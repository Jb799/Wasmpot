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
use chrono::{DateTime, Utc, Duration, Datelike, Timelike};
use url::form_urlencoded;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

mod rules;
use rules::{MethodType, ParamValue, Param, get_rules};

// Cache rules using Lazy and tokio::sync::Mutex for async safety
static RULES: Lazy<Mutex<Vec<rules::Rule>>> = Lazy::new(|| {
    Mutex::new(get_rules())
});

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

fn display_logs(query_pairs: &HashMap<String, String>, param: &Param, path: &str) {
    let utc_now: DateTime<Utc> = Utc::now();
    let paris_offset = if is_dst(utc_now) { 2 } else { 1 };
    let paris_now = utc_now + Duration::hours(paris_offset);
    let query_string = format_query_params(query_pairs);
    println!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}", paris_now, param.id, param.flag, path, query_string, "127.0.0.1", 0.0, 0.0);
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

fn redirect_response(redirect_url: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", HeaderValue::from_str(redirect_url).unwrap())
        .body(Body::empty())
        .unwrap()
}

async fn extract_body_params(req: &mut Request<Body>) -> HashMap<String, String> {
    let whole_body = body::to_bytes(req.body_mut()).await.unwrap();
    form_urlencoded::parse(&whole_body).into_owned().collect()
}

async fn echo(mut req: Request<Body>, web_res_port: u16, web_res_name: String) -> Result<Response<Body>, hyper::Error> {
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
                        display_logs(&params, param, &path);
                        if let Some(redirect_url) = &param.redirect {
                            return Ok(redirect_response(redirect_url));
                        }
                        break;
                    },
                    _ => {
                        display_logs(&params, param, &path);
                        if let Some(redirect_url) = &param.redirect {
                            return Ok(redirect_response(redirect_url));
                        }
                        break;
                    }
                }
            }
        }
    }

    fetch_resource(&path, web_res_port, web_res_name).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let mut wasi_port: u16 = 8000;
    let mut actix_port: u16 = 8888;
    let mut actix_name: String = "localhost".to_string();

    if args.len() >= 4 {
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
    } else {
        println!("Missing arguments (WASI_PORT, ACTIX_PORT, ACTIX_NAME)");
        return Err("Missing arguments (WASI_PORT, ACTIX_PORT, ACTIX_NAME)".into());
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], wasi_port));
    let listener = TcpListener::bind(addr).await?;

    println!("Timestamp,ID,FLAG,Endpoint,Payload,IP,Latitude,Longitude");

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        let actix_name_clone = actix_name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(move |req| {
                echo(req, actix_port, actix_name_clone.clone())
            })).await {
                println!("Error serving connection: {}", err);
            }
        });
    }
}
