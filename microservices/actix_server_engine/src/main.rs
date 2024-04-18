use actix_web::{web, App, HttpResponse, HttpServer, Result, http::StatusCode};
use async_std::fs;
use std::env;
use std::sync::{Arc, Mutex};
use serde_json::Value;
use std::path::Path;

use walkdir::WalkDir;
use std::error::Error;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;

#[derive(Clone)]
struct Endpoints {
    url: String,
    content_file: String,
    req_type: String,
    status_code: u16,
    headers: Vec<Vec<String>>,
}

/* ############## FUNCTIONS ############## */
fn generate_random_string(len: usize) -> String {
    let rng = thread_rng();
    let random_string: String = rng.sample_iter(&Alphanumeric).take(len).map(char::from).collect();
    random_string
}

async fn read_json_file(file_path: &str) -> Result<Endpoints, Box<dyn Error>> {
    let json_content = async_std::fs::read_to_string(file_path).await?;
    let parsed_json: Value = serde_json::from_str(&json_content)?;

    // Extract values from JSON
    let url = parsed_json["url"].as_str().ok_or("URL field missing")?.to_string();
    let content_file = parsed_json["content_file"].as_str().unwrap_or("").to_string();
    let req_type = parsed_json["type"].as_str().ok_or("Request type field missing")?.to_string();
    let status_code = parsed_json["status_code"].as_u64().ok_or("Status code field missing")? as u16;

    // Extract headers
    let headers = match parsed_json["headers"].as_object() {
        Some(header_map) => {
            header_map.iter().map(|(key, value)| {
                vec![key.to_string(), value.as_str().unwrap_or_default().to_string()]
            }).collect()
        }
        None => vec![],
    };

    // Create and return a new Endpoints instance
    Ok(Endpoints {
        url,
        content_file,
        req_type,
        status_code,
        headers,
    })
}

/* ############## API/INIT FUNCTIONS ############## */
async fn get_endpoints() -> Vec<Endpoints> {
    let mut endpoints: Vec<Endpoints> = Vec::new();

    // Read all JSON files in the sources folder:
    for entry in WalkDir::new("./src/sources") {
        if let Ok(entry) = entry {
            if let Some(extension) = entry.path().extension() {
                if extension == "json" {
                    match read_json_file(entry.path().to_str().unwrap()).await {
                        Ok(endpoint) => {
                            endpoints.push(endpoint);
                        }
                        Err(err) => {
                            eprintln!("Error reading JSON file: {}", err);
                        }
                    }
                }
            }
        }
    }

    // let new_endpoint = Endpoints {
    //     path: "/".to_string(),
    //     content_path: "/html/index.html".to_string(),
    //     req_type: "GET".to_string(),
    //     status_code: 200,
    //     headers: vec![vec!["content-type".to_string(), "text/html".to_string()]],
    // };
    // endpoints.push(new_endpoint);

    endpoints
}

async fn default_api(req: actix_web::HttpRequest, web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>, endpoints: web::Data<Arc<Mutex<Vec<Endpoints>>>>) -> Result<HttpResponse> {
    let mut requested_path = req.uri().path().to_owned();
    let endpoints = endpoints.lock().unwrap();

    // Remove trailing slash from the requested path
    if requested_path.len() > 1 {
        if let Some(stripped) = requested_path.strip_suffix("/") {
            requested_path = stripped.to_string();
        }
    }

    println!("\x1B[34m[🌐] Requested path: {}\x1B[0m", requested_path);

    for endpoint in endpoints.iter() {
        if endpoint.url == requested_path {
            println!("\x1B[32m[✅] Endpoint found: {}\x1B[0m", endpoint.url);

            let status = StatusCode::from_u16(endpoint.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let mut response = HttpResponse::build(status);

            for header in &endpoint.headers {
                let mut header_rep = header[1].to_string();
                header_rep = header_rep.replace("{PORT}", &web_wasi_port.to_string());
                header_rep = header_rep.replace("{ADDR}", &web_wasi_addr.to_string());

                response.insert_header((header[0].to_string(), header_rep.clone()));
            }

            if !endpoint.content_file.is_empty() {
                println!("\x1B[32m[✅] Content file found: {}\x1B[0m", "./src/sources/".to_owned() + &endpoint.content_file);

                let mut file_content = fs::read("./src/sources/".to_owned() + &endpoint.content_file).await?;

                if let Some(extension) = Path::new(&endpoint.content_file).extension() {
                    if let Some(extension_str) = extension.to_str() {
                        if extension_str == "html" || extension_str == "css" || extension_str == "js" {
                            let mut str_content = String::from_utf8_lossy(&file_content).to_string();
                            str_content = str_content.replace("{PORT}", &web_wasi_port.to_string());
                            str_content = str_content.replace("{ADDR}", &web_wasi_addr.to_string());
                            file_content = str_content.as_bytes().to_vec();
                        }
                    }
                }

                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                let is_gzip = endpoint.headers.iter().any(|header| header[0] == "Content-Encoding" && header[1].to_lowercase() == "gzip");

                if is_gzip {
                    encoder.write_all(&file_content)?;
                    let compressed_content = encoder.finish()?;
                    return Ok(response.body(compressed_content));
                } else {
                    return Ok(response.body(file_content));
                }
            }else{
                return Ok(response.finish());
            }
        }
    }

    println!("\x1B[31m[❌] Endpoint not found\x1B[0m");

    // Return 404 if no endpoint is found, use the default 404 page:
    let path = "./src/sources/html/404.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{ADDR}", &web_wasi_addr.to_string());

    let mut response = HttpResponse::NotFound();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

/* ############## MAIN FUNCTION ############## */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let web_port: u16 = env::var("WEB_PORT").unwrap_or("8888".to_string()).parse().unwrap();
    let web_wasi_port: u16 = env::var("WEB_WASI_PORT").unwrap_or("8888".to_string()).parse().unwrap(); // .unwrap_or("8000".to_string()).parse().unwrap();
    let web_wasi_addr: String = env::var("WEB_ADDR").unwrap_or("localhost".to_string()).parse().unwrap();

    // Endpoints initialization:
    let endpoints: Arc<Mutex<Vec<Endpoints>>> = Arc::new(Mutex::new(get_endpoints().await));

    println!("\n\x1B[32m---------------------------------------------\x1B[0m");
    println!("\x1B[1;32m####### ✨ WasmPot2 Resource Server ✨ #######\x1B[0m\n");
    println!("\x1B[32m[📡] Listening on http://{}:{}/\x1B[0m", web_wasi_addr, web_port);
    println!("\x1B[32m[✅] Running !\x1B[0m");
    println!("\x1B[32m---------------------------------------------\x1B[0m\n\n");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(web_wasi_port))
            .app_data(web::Data::new(web_wasi_addr.to_string()))
            .app_data(web::Data::new(endpoints.clone()))
            .default_service(
                web::route().to(default_api)
            )
    })
    .bind(("0.0.0.0", web_port))?
    .run()
    .await
}