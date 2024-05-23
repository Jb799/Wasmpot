use std::net::SocketAddr;
use std::env;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Response, Uri, StatusCode};
use hyper::body;
use hyper::header::HeaderValue;
use regex::Regex;
use tokio::net::TcpListener;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration, Datelike, Timelike};
use url::form_urlencoded;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

mod rules;
use rules::{MethodType, ParamValue, Param, get_rules};
use crate::rules::{get_response_rules, ResponseModification, ResponseRule};

// Cache rules using Lazy and tokio::sync::Mutex for async safety
static RULES: Lazy<Mutex<Vec<rules::Rule>>> = Lazy::new(|| {
    Mutex::new(get_rules())
});

#[derive(Debug, Clone)]
struct ClientInfo {
    ip: String,
}

#[derive(Serialize)]
struct LogEntry {
    timestamp: String,
    id: u32,
    flag: u8,
    path: String,
    query_string: String,
    client_ip: String,
    country: String,
    loc: String,
}

async fn fetch_resource(path: &str, web_res_port: u16, web_res_name: String) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    let uri = format!("http://{}:{}{}", web_res_name, web_res_port, path);
    let uri = uri.parse::<Uri>().unwrap();

    client.get(uri).await
}

fn is_static_resource(uri: &Uri) -> bool {
    if let Some(path) = uri.path().split('?').next() {
        let ext = path.split('.');

        if ext.clone().count() == 1 {
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
    let hour = datetime.hour();
    (month > 3 && month < 10) || (month == 3 && hour >= 1) || (month == 10 && hour < 1)
}

async fn fetch_ip_info(ip: &str) -> (String, String) {
    let client = Client::new();
    let uri = format!("https://ipinfo.io/{}/json", ip).parse::<Uri>().expect("Failed to parse URI");
    match client.get(uri).await {
        Ok(response) => {
            let body = hyper::body::to_bytes(response.into_body()).await.expect("Failed to read response body");
            let data: Value = serde_json::from_slice(&body).expect("Failed to parse JSON");
            let loc = data.get("loc").and_then(|v| v.as_str()).unwrap_or("Unknown Location").to_string();
            let country = data.get("country").and_then(|v| v.as_str()).unwrap_or("Unknown Country").to_string();
            (loc, country)
        },
        Err(_) => ("API request failed".to_string(), "API request failed".to_string())
    }
}

async fn display_logs(query_pairs: &HashMap<String, String>, param: &Param, path: &str, admin_name: String, admin_port: u16, req: &Request<Body>) {
    let client_ip = req.headers().get("CF-Connecting-IP")
                     .and_then(|v| v.to_str().ok())
                     .unwrap_or("Unknown IP");
    
    let mut loc = "Unknown Location".to_string();
    let mut country = "Unknown Country".to_string();

    if client_ip != "Unknown IP" {
        let ip_info = fetch_ip_info(client_ip).await;
        loc = ip_info.0;
        country = ip_info.1;
    }

    let utc_now: DateTime<Utc> = Utc::now();
    let paris_offset = if is_dst(utc_now) { 2 } else { 1 };
    let paris_now = utc_now + Duration::hours(paris_offset);
    let query_string = format_query_params(query_pairs);

    let log_entry = LogEntry {
        timestamp: paris_now.to_rfc3339(),
        id: param.id.clone(),
        flag: param.flag.clone(),
        path: path.to_string(),
        query_string: query_string.clone(),
        client_ip: client_ip.to_string(),
        country: country.to_string(),
        loc: loc.to_string(),
    };

    println!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}", paris_now, param.id, param.flag, path, query_string, client_ip, country, loc);

    let client = Client::new();

    let uri = format!("http://{}:{}/save_log", admin_name, admin_port).parse::<Uri>().unwrap();
    let json = serde_json::to_string(&log_entry).unwrap();
    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap();

    match client.request(req).await {
        Ok(response) => {
            if !response.status().is_success() {
                eprintln!("Failed to send log entry: {}", response.status());
            }
        }
        Err(e) => {
            eprintln!("Error sending log entry: {:?}", e);
        }
    }
}

fn format_query_params(query_pairs: &HashMap<String, String>) -> String {
    query_pairs.iter()
        .filter(|(key, _)| !key.is_empty())
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

fn is_value_valid(expected: &ParamValue, value: &str) -> bool {
    match expected {
        ParamValue::Regex(regex) => regex.is_match(value),
        ParamValue::Exact(exact) => exact == value,
        ParamValue::AlwaysFail => false,
    }
}

async fn extract_body_params(req: &mut Request<Body>) -> HashMap<String, String> {
    let whole_body = body::to_bytes(req.body_mut()).await.unwrap();
    form_urlencoded::parse(&whole_body).into_owned().collect()
}

async fn apply_response_modifications(
    mut response: Response<Body>,
    request_path: &str,
    method: MethodType,
    params: &mut HashMap<String, String>
) -> Response<Body> {
    let response_rules = get_response_rules();

    if let Some(rule) = response_rules.iter().find(|rule| rule.url == request_path && rule.method == method) {
        let body_bytes = hyper::body::to_bytes(response.body_mut()).await.unwrap();
        let mut body = String::from_utf8(body_bytes.to_vec()).unwrap_or_else(|_| String::from(""));

        for modification in &rule.modifications {
            match modification {
                ResponseModification::Sanitize { param_name, regex } => {
                    if let Some(value) = params.get(param_name) {
                        let sanitized_value = regex.replace_all(value, "").to_string();
                        params.insert(param_name.clone(), sanitized_value);
                    }
                },
                ResponseModification::Replace { placeholder, param_name } => {
                    if let Some(value) = params.get(param_name) {
                        body = body.replace(placeholder, value);
                    } else {
                        body = body.replace(placeholder, "");
                    }
                },
            }
        }

        *response.body_mut() = Body::from(body);
    }

    response
}


async fn echo(mut req: Request<Body>, web_res_port: u16, web_res_name: String, admin_name: String, admin_port: u16) -> Result<Response<Body>, hyper::Error> {
    let method = if req.method() == &Method::GET { MethodType::GET } else { MethodType::POST };
    let path = req.uri().path().to_owned();
    let query_string = req.uri().query().unwrap_or_default().to_owned();

    let rules = RULES.lock().await;
    let mut params = HashMap::new();

    if !is_static_resource(req.uri()) {
        if let Some(rule) = rules.iter().find(|rule| &rule.url == &path && rule.method == method) {
            if method == MethodType::POST {
                params = extract_body_params(&mut req).await;
            } else {
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
                        display_logs(&params, param, &path, admin_name.clone(), admin_port, &req).await;
                        if let Some(redirect_url) = &param.redirect {
                            let response = fetch_resource(&redirect_url, web_res_port, web_res_name).await?;
                            return Ok(response);
                        }
                        break;
                    },
                    _ => {
                        display_logs(&params, param, &path, admin_name.clone(), admin_port, &req).await;
                        if let Some(redirect_url) = &param.redirect {
                            let response = fetch_resource(&redirect_url, web_res_port, web_res_name).await?;
                            return Ok(response);
                        }
                        break;
                    }
                }
            }
        }
    }else{
        let response = fetch_resource(&path, web_res_port, web_res_name).await?;
        return Ok(response);
    }

    let response = fetch_resource(&path, web_res_port, web_res_name).await?;
    let final_response = apply_response_modifications(response, &path, method, &mut params).await;
    Ok(final_response)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let mut wasi_port: u16 = 8000;
    let mut actix_port: u16 = 8888;
    let mut actix_name: String = "localhost".to_string();
    let mut admin_port: u16 = 8068;
    let mut admin_name: String = "localhost".to_string();

    if args.len() >= 6 {
        if let Ok(port) = args[1].parse::<u16>() {
            wasi_port = port;
        } else {
            println!("Invalid value for WASI_PORT");
            return Err("Invalid value for WASI_PORT".into());
        }
    
        if let Ok(res_port) = args[2].parse::<u16>() {
            actix_port = res_port;
        } else {
            println!("Invalid value for ACTIX_PORT");
            return Err("Invalid value for ACTIX_PORT".into());
        }
        
        actix_name = args[3].clone();

        if let Ok(arg_admin_port) = args[4].parse::<u16>() {
            admin_port = arg_admin_port;
        } else {
            println!("Invalid value for ADMIN_PORT");
            return Err("Invalid value for ADMIN_PORT".into());
        }

        admin_name = args[5].clone();
    } else {
        println!("Missing arguments (WASI_PORT, ACTIX_PORT, ACTIX_NAME, ADMIN_PORT, ADMIN_NAME)");
        return Err("Missing arguments (WASI_PORT, ACTIX_PORT, ACTIX_NAME, ADMIN_PORT, ADMIN_NAME)".into());
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], wasi_port));
    let listener = TcpListener::bind(addr).await?;

    println!("Timestamp,ID,FLAG,Endpoint,Payload,IP,Country,Latitude,Longitude");

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        let actix_name_clone = actix_name.clone();
        let admin_name_clone = admin_name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(move |req| {
                echo(req, actix_port, actix_name_clone.clone(), admin_name_clone.clone(), admin_port)
            })).await {
                println!("Error serving connection: {}", err);
            }
        });
    }
}
