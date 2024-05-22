use std::net::SocketAddr;
use std::env;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Response, Uri, StatusCode, body};
use hyper::header::HeaderValue;

use regex::Regex;

use tokio::net::TcpListener;

use serde::Deserialize;

use std::collections::HashMap;

mod rules;
use rules::{MethodType, ParamValue, get_rules, Param};

use chrono::{DateTime, Utc, Duration, Datelike, Timelike};

use url::form_urlencoded;

/* Client info */
#[derive(Debug, Clone)]
struct ClientInfo {
    ip: String,
}

async fn fetch_resource(path: &str, web_res_port: u16, web_res_name: String) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    let uri = format!("http://{}:{}{}", web_res_name, web_res_port, path);
    let uri = uri.parse::<Uri>().unwrap();

    client.get(uri).await
}

/// Validates the value against the expected parameter value.
fn is_value_valid(expected: &ParamValue, value: &str) -> bool {
    match expected {
        ParamValue::Regex(regex) => regex.is_match(value),
        ParamValue::Exact(exact) => exact == value,
        ParamValue::AlwaysFail => false,
    }
}

/// Constructs a redirect response.
fn redirect_response(redirect_url: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", HeaderValue::from_str(redirect_url).unwrap())
        .body(Body::empty())
        .unwrap()
}

fn is_static_resource(uri: &Uri) -> bool {
    if let Some(path) = uri.path().split('?').next() {
        let ext = path.split('.');

        if (ext.clone().count() == 1) {
            return false;
        }

        match ext.last().unwrap_or("") {
            "html" | "php" | "js" => false,
            _ => true,
        }
    } else {
        false
    }
}

fn is_dst(datetime: DateTime<Utc>) -> bool {
    let month = datetime.month();
    let hour = datetime.hour(); // Now this should work correctly with Timelike in scope
    (month > 3 && month < 10) || (month == 3 && hour >= 1) || (month == 10 && hour < 1)
}

fn format_query_params(query_pairs: &HashMap<String, String>) -> String {
    query_pairs.iter()
        .filter(|(key, _)| key.as_str() != "")
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

fn display_logs(query_pairs: &HashMap<String, String>, param: &Param, path: &str){
    let utc_now: DateTime<Utc> = Utc::now();
    let paris_offset = if is_dst(utc_now) { 2 } else { 1 };
    let paris_now = utc_now + Duration::hours(paris_offset);
    let query_string = format_query_params(&query_pairs);

    println!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}", paris_now, param.id, param.flag, path, query_string, "remote_addr", 0.0, 0.0);
}

async fn extract_body_params(req: &mut Request<Body>) -> HashMap<String, String> {
    let whole_body = body::to_bytes(req.body_mut()).await.unwrap();
    form_urlencoded::parse(&whole_body).into_owned().collect()
}

async fn echo(mut req: Request<Body>, web_res_port: u16, web_res_name: String) -> Result<Response<Body>, hyper::Error> {
    let method = if req.method() == &Method::GET { MethodType::GET } else { MethodType::POST };
    // Extract URI early and convert to String to avoid borrowing issues
    let path = req.uri().path().to_owned();
    let query_string = req.uri().query().unwrap_or_default().to_owned();
    
    let rules = get_rules();
    let mut params = HashMap::new();

    if !is_static_resource(req.uri()) {
        if let Some(rule) = rules.iter().find(|rule| &rule.url == &path && rule.method == method){
            if method == MethodType::POST {
                // Since we're dealing with POST, extract params from body
                params = extract_body_params(&mut req).await;  // This mutably borrows req
            } else {
                // Parse query string for GET parameters
                params = query_string.split('&').filter_map(|pair| {
                    let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
                    Some((key.to_string(), value.to_string()))
                }).collect::<HashMap<_, _>>();
            }
    
            for param in &rule.query_params {
                match params.get(&param.name) {
                    Some(value) if is_value_valid(&param.value, value) => continue,
                    Some(_) if !param.required => continue,
                    None if param.required => {
                        display_logs(&params, param, &path);
                        if let Some(redirect_url) = &param.redirect {
                            return Ok(redirect_response(redirect_url));
                        }
                    },
                    _ => {
                        display_logs(&params, param, &path);
                        if let Some(redirect_url) = &param.redirect {
                            return Ok(redirect_response(redirect_url));
                        }
                    }
                }
            }
        }
    }

    // If the request passes the rule checks or no specific rules apply, fetch and return the resource.
    fetch_resource(&path, web_res_port, web_res_name).await
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Récupérer les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();
    let mut web_port: u16 = env::var("WEBWASI_PORT").unwrap_or("8000".to_string()).parse().unwrap();
    let mut web_res_port: u16 = env::var("WEBRES_PORT").unwrap_or("8888".to_string()).parse().unwrap();
    let mut web_res_name: String = env::var("WEBRES_ADDR").unwrap_or("localhost".to_string()).parse().unwrap();

    let mut clients: Vec<ClientInfo> = Vec::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], web_port));
    let listener = TcpListener::bind(addr).await?;

    println!("Timestamp,ID,FLAG,Endpoint,Payload,IP,Latitude,Longitude");

    loop {
        let (stream, _) = listener.accept().await?;
        // let client_ip = stream.peer_addr().unwrap().ip().to_string();
    
        // Vérifier si un client avec la même adresse IP existe déjà
        // if !clients.iter().any(|client| client.ip == client_ip) {
        //     let client_info = ClientInfo {
        //         ip: client_ip.clone(),
        //     };
    
        //     clients.push(client_info);
        //     // println!("\x1B[1;93m[🦈] New connection from {}\x1B[0m\n", client_ip);
        // }
    
        let web_res_name_clone = web_res_name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(move |req| {
                echo(req, web_res_port, web_res_name_clone.clone())
            })).await {
                println!("Error serving connection: {}", err);
            }
        });
    }
}