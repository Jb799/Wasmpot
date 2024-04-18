use std::net::SocketAddr;
use std::env;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Response, Uri};
use hyper::header::HeaderValue;

use tokio::net::TcpListener;

use serde::Deserialize;
use serde_urlencoded::from_str;
use serde_urlencoded::to_string;

use regex::Regex;
use std::collections::HashMap;

mod rules;
use rules::{Rule, Method, ParamValue, get_rules};

/* Client info */
#[derive(Debug, Clone)]
struct ClientInfo {
    ip: String,
}

/* Auth Form */
#[derive(Debug, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

/* Notifications */
enum Notif {
    RPost,
    RGet,
    Err,
}

async fn get_resource(path: String, web_res_port: u16, web_res_name: String) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    let uri = format!("http://{}:{}{}", web_res_name, web_res_port, path);
    let uri = uri.parse::<Uri>().unwrap();

    let res = client.get(uri).await?;

    Ok( res )
}

fn draw_notif(type_notif: Notif, txt: &str) {
    match type_notif {
        Notif::RPost => println!("\x1B[41m[📝] POST /{}\x1B[0m", txt),
        Notif::RGet => println!("\x1B[34m[👀] GET /{}\x1B[0m", txt),
        Notif::Err => println!("\x1B[1;31m[🚨] {}\x1B[0m", txt),
    }
}

fn xss_filter(input: &str) -> String {
    let mut filtered = String::new();

    for c in input.chars() {
        if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '@' || c == ':'{
            filtered.push(c);
        }
    }

    filtered
}

async fn echo(req: Request<Body>, web_res_port: u16, web_res_name: String) -> Result<Response<Body>, hyper::Error> {
    let method = if req.method() == &HttpMethod::GET { Method::GET } else { Method::POST };
    let uri = req.uri();
    let path = uri.path();
    let rules = get_rules();

    let rule = rules.iter().find(|rule| rule.url == path && rule.method == method);

    if let Some(rule) = rule {
        let query_pairs = uri.query().unwrap_or("").split('&').map(|pair| {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            (key.to_string(), value.to_string())
        }).collect::<HashMap<_, _>>();

        for param in &rule.query_params {
            if let Some(value) = query_pairs.get(&param.name) {
                let is_valid = match &param.value {
                    ParamValue::Regex(regex) => regex.is_match(value),
                    ParamValue::Exact(expected) => expected == value,
                };
                if !is_valid {
                    if let Some(redirect_url) = &param.redirect {
                        return Ok(Response::builder()
                            .status(StatusCode::FOUND)
                            .header("Location", HeaderValue::from_str(redirect_url).unwrap())
                            .body(Body::empty())?);
                    }
                }
            }
        }
    }

    let response = get_resource(path.to_string(), web_res_port, web_res_name).await?;
    let status_code = response.status();
    let body = response.into_body();

    Ok(Response::builder()
        .status(status_code)
        .body(body)?)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Récupérer les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();
    let mut web_port: u16 = 8000;
    let mut web_res_port: u16 = 8888;
    let mut web_res_name: String = "localhost".to_string();

    if args.len() >= 4 {
        if let Ok(port) = args[1].parse::<u16>() {
            web_port = port;
        } else {
            draw_notif(Notif::Err, "Invalid value for WEB_PORT");
            return Err("Invalid value for WEB_PORT".into());
        }
    
        if let Ok(res_port) = args[2].parse::<u16>() {
            web_res_port = res_port;
        } else {
            draw_notif(Notif::Err, "Invalid value for WEB_RES_PORT");
            return Err("Invalid value for WEB_RES_PORT".into());
        }
    
        web_res_name = args[3].clone();
    } else {
        draw_notif(Notif::Err, "Missing arguments");
        return Err("Missing arguments".into());
    }

    let mut clients: Vec<ClientInfo> = Vec::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], web_port));
    let listener = TcpListener::bind(addr).await?;

    println!("\n\x1B[32m---------------------------------------------\x1B[0m");
    println!("\x1B[1;32m####### 🐝 WasmPot2 Wasi Server 🐝 #######\x1B[0m\n");
    println!("\x1B[32m[📡] Listening on http://{:?}/\x1B[0m", addr);
    println!("\x1B[32m[📡] Connected with http://{}:{}/\x1B[0m", web_res_name, web_res_port);
    println!("\x1B[32m[✅] Running !\x1B[0m");
    println!("\x1B[32m---------------------------------------------\x1B[0m\n\n");

    loop {
        let (stream, _) = listener.accept().await?;
        let client_ip = stream.peer_addr().unwrap().ip().to_string();
    
        // Vérifier si un client avec la même adresse IP existe déjà
        if !clients.iter().any(|client| client.ip == client_ip) {
            let client_info = ClientInfo {
                ip: client_ip.clone(),
            };
    
            clients.push(client_info);
            println!("\x1B[1;93m[🦈] New connection from {}\x1B[0m\n", client_ip);
        }
    
        let web_res_name_clone = web_res_name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(move |req| {
                echo(req, web_res_port, web_res_name_clone.clone())
            })).await {
                draw_notif(Notif::Err, &format!("Error serving connection: {}.", err));
            }
        });
    }
}