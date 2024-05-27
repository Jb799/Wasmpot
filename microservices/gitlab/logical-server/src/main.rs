use std::net::SocketAddr;
use std::env;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Response, Uri};
use hyper::StatusCode;
use hyper::body;
use tokio::net::TcpListener;
use serde::Serialize;
use serde_json;
use crate::serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration, Datelike, Timelike};
use url::form_urlencoded;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

mod rules;
use rules::{MethodType, ParamValue, Param, get_rules};
use crate::rules::{get_response_rules, ResponseModification};

// Cache rules using Lazy and tokio::sync::Mutex for async safety
static RULES: Lazy<Mutex<Vec<rules::Rule>>> = Lazy::new(|| {
    Mutex::new(get_rules())
});

#[derive(Serialize)]
struct LogEntry {
    timestamp: String,
    id: u32,
    flag: u8,
    method: String,
    path: String,
    client_useragent: String,
    query_string: String,
    client_ip: String,
    isp: String,
    country: String,
    city: String,
    lat: String,
    long: String,
}

#[derive(Clone, Debug)]
pub struct Localisation {
    pub ip: String,
    pub lat: String,
    pub long: String,
    pub country: String,
    pub city: String,
    pub isp: String,
}

lazy_static! {
    static ref GLOBAL_LOCALISATIONS: Mutex<Vec<Localisation>> = Mutex::new(vec![]);
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
            "html" | "php" => false,
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

async fn fetch_ip_info(ip: &str) -> Result<(String, String, String, String, String), Box<dyn std::error::Error>> {
    let client = Client::new();
    let uri = format!("http://ip-api.com/json/{}", ip).parse::<Uri>()?;
    let response = client.get(uri).await?;

    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_string = String::from_utf8(body_bytes.to_vec())?;

    if let Ok(data) = serde_json::from_str::<Value>(&body_string) {
        let lat = data.get("lat").and_then(|v| v.as_str()).unwrap_or("0").to_string();
        let long = data.get("lon").and_then(|v| v.as_str()).unwrap_or("0").to_string();
        let country = data.get("country").and_then(|v| v.as_str()).unwrap_or("Unknown Country").to_string();
        let city = data.get("city").and_then(|v| v.as_str()).unwrap_or("Unknown City").to_string();
        let isp = data.get("isp").and_then(|v| v.as_str()).unwrap_or("Unknown ISP").to_string();
        Ok((lat, long, country, city, isp))
    } else {
        Err("Failed to parse JSON".into())
    }
}

async fn display_logs(
    query_pairs: Option<&HashMap<String, String>>, 
    param: Option<&Param>, 
    path: &str, 
    admin_name: String, 
    admin_port: u16, 
    req: &Request<Body>,
    _flag: u8,
    _id: u32
) {
    let mut locs = GLOBAL_LOCALISATIONS.lock().await;

    let method = req.method().as_str().to_string();

    let client_ip = req.headers().get("CF-Connecting-IP")
                     .and_then(|v| v.to_str().ok())
                     .unwrap_or("37.166.165.255");

    let client_useragent = req.headers().get("User-Agent")
                     .and_then(|v| v.to_str().ok())
                     .unwrap_or("Unknown User-Agent");
    
    let mut lat: String = "0".to_string();
    let mut long: String = "0".to_string();
    let mut country = "Unknown Country".to_string();
    let mut city = "Unknown City".to_string();
    let mut isp = "Unknown Isp".to_string();

    if let Some(_loc) = locs.iter().find(|l| l.ip == client_ip) {
        lat = _loc.clone().lat;
        long = _loc.clone().long;
        country = _loc.country.clone();
    } else if client_ip != "Unknown IP" {
        if let Ok(ip_info) = fetch_ip_info(client_ip).await {
            lat = ip_info.0;
            long = ip_info.1;
            country = ip_info.2;
            city = ip_info.3;
            isp = ip_info.4;

            let new_loc = Localisation {
                ip: client_ip.to_string(),
                lat: lat.clone(),
                long: long.clone(),
                country: country.clone(),
                city: city.clone(),
                isp: isp.clone(),
            };
            locs.push(new_loc);
        } else {
            eprintln!("Failed to fetch or parse IP information");
        }
    }

    let utc_now: DateTime<Utc> = Utc::now();
    let paris_offset = if is_dst(utc_now) { 2 } else { 1 };
    let paris_now = utc_now + Duration::hours(paris_offset);

    let query_string = query_pairs
        .map_or(String::new(), |pairs| format_query_params(pairs));

    let (id, flag) = param
        .map(|p| (p.id, p.flag))
        .unwrap_or((_id, _flag));

    let log_entry = LogEntry {
        timestamp: paris_now.to_rfc3339(),
        id,
        flag,
        method: method.clone(),
        path: path.to_string(),
        query_string: query_string.clone(),
        client_useragent: client_useragent.to_string(),
        client_ip: client_ip.to_string(),
        isp: isp.clone(),
        country: country.clone(),
        city: city.clone(),
        lat: lat.clone(),
        long: long.clone(),
    };

    println!("{},{},{},{},{},{},{},{},{},{},{}, {}, {}", paris_now, id, flag, method, path, query_string, client_useragent, client_ip, isp, country, city, lat, long);

    let client = Client::new();
    let uri = format!("http://{}:{}/save_log", admin_name, admin_port).parse::<Uri>().unwrap();
    let json = serde_json::to_string(&log_entry).unwrap();
    let request = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap();

    tokio::spawn(async move {
        let _ = client.request(request).await;
    });
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
    path: &str,
    method: MethodType,
    params: &HashMap<String, String>
) -> Result<Response<Body>, hyper::Error> {
    let rules = get_response_rules();
    
    if let Some(rule) = rules.iter().find(|rule| rule.url == path && (rule.method == method || rule.method == MethodType::ANY)) {
        let status_code = response.status();
        let headers = response.headers().clone();

        let mut body_bytes = hyper::body::to_bytes(response.body_mut()).await?;
        let mut body = String::from_utf8(body_bytes.to_vec()).unwrap_or_default();

        let mut params_copy = params.clone();

        for modification in &rule.modifications {
            match modification {
                ResponseModification::Sanitize { param_name, regex } => {
                    if let Some(value) = params_copy.get(param_name.as_str()) {
                        let sanitized_value = regex.replace_all(value, "");
                        params_copy.insert(param_name.clone(), sanitized_value.to_string());
                    } else {
                        params_copy.insert(param_name.clone(), String::new());
                    }
                },
                ResponseModification::Replace { placeholder, param_name } => {
                    if let Some(value) = params_copy.get(param_name.as_str()) {
                        if body.contains(placeholder.as_str()) {
                            body = body.replace(placeholder.as_str(), value);
                        }
                    } else {
                        if body.contains(placeholder.as_str()) {
                            body = body.replace(placeholder.as_str(), "");
                        }
                    }
                }
            }
        }

        let mut response = Response::builder()
        .status( status_code )
        .body( Body::from(body) )
        .unwrap();

        for (name, value) in headers.iter() {
            if name == "content-length" || name == "date"{
                continue;
            }

            response.headers_mut().insert(name, value.clone());
        }

        return Ok( response );
    }

    Ok( response )
}

async fn echo(mut req: Request<Body>, web_res_port: u16, web_res_name: String, admin_name: String, admin_port: u16) -> Result<Response<Body>, hyper::Error> {
    let method = if req.method() == &Method::GET { MethodType::GET } else { MethodType::POST };
    let path = req.uri().path().to_owned();
    let query_string = req.uri().query().unwrap_or_default().to_owned();

    let rules = RULES.lock().await;
    let mut params = HashMap::new();
    let mut filtered: bool = false;

    if !is_static_resource(req.uri()) {
        if method == MethodType::POST {
            params = extract_body_params(&mut req).await;
        } else {
            params = query_string.split('&').map(|pair| {
                let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
                (key.to_string(), value.to_string())
            }).collect::<HashMap<_, _>>();
        }

        if let Some(rule) = rules.iter().find(|rule| rule.url_pattern.is_match(&path) && ( rule.method == MethodType::ANY || rule.method == method) ) {
            if rule.query_params.is_empty() {
                display_logs(
                    Some(&params),
                    None,
                    &path,
                    admin_name.clone(),
                    admin_port,
                    &req,
                    rule.flag.unwrap_or(0),
                    rule.id.unwrap_or(0)
                ).await;

                if let Some(redirect_url) = &rule.redirect {
                    let response = fetch_resource(redirect_url, web_res_port, web_res_name).await?;
                    return Ok(response);
                }else{
                    let response = fetch_resource(&path, web_res_port, web_res_name).await?;
                    let final_response = apply_response_modifications(response, &path, method, &mut params).await?;
                    return Ok( final_response );
                }
            }else{
                for param in &rule.query_params {
                    match params.get(&param.name) {
                        Some(value) if is_value_valid(&param.value, value) => continue,
                        Some(_) if !param.required => continue,
                        None if param.required => {
                            display_logs(Some(&params), Some(param), &path, admin_name.clone(), admin_port, &req, 0, 0).await;
                            if let Some(redirect_url) = &param.redirect {
                                let response = fetch_resource(&redirect_url, web_res_port, web_res_name).await?;
                                return Ok(response);
                            }
                            filtered = true;
                            break;
                        },
                        _ => {
                            display_logs(Some(&params), Some(param), &path, admin_name.clone(), admin_port, &req, 0, 0).await;
                            if let Some(redirect_url) = &param.redirect {
                                let response = fetch_resource(&redirect_url, web_res_port, web_res_name).await?;
                                return Ok(response);
                            }
                            filtered = true;
                            break;
                        }
                    }
                }
            }
        }

        if !filtered{
            display_logs(Some(&params), None, &path, admin_name.clone(), admin_port, &req, 0, 0).await;
        }
    }else{
        let response = fetch_resource(&path, web_res_port, web_res_name).await?;
        return Ok(response);
    }

    let response = fetch_resource(&path, web_res_port, web_res_name).await?;
    let final_response = apply_response_modifications(response, &path, method, &mut params).await?;

    Ok( final_response )
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

    println!("Timestamp,ID,FLAG,Method,Endpoint,Payload,IP,Country,Latitude_Longitude");

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        let actix_name_clone = actix_name.clone();
        let admin_name_clone = admin_name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(move |req| {
                echo(req, actix_port, actix_name_clone.clone(), admin_name_clone.clone(), admin_port)
            })).await {
                // println!("Error serving connection: {}", err);
            }
        });
    }
}