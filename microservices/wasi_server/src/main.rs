use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Response, Uri};

use tokio::net::TcpListener;

mod functions;
use functions::set_attributs_header;
use functions::get_page_id;

use serde::Deserialize;
use serde_urlencoded::from_str;

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

/* STATICS */
static WEB_PORT: u16 = 8000;
static WEB_RESOURCES_PORT: u16 = 8888;

async fn get_resource(path: String) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    let uri = format!("http://localhost:{}{}", WEB_RESOURCES_PORT, path);
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

fn check_query_params( uri: &Uri ) -> Result<(String, String), String> {
    let mut tab_id = "".to_string();

    match uri.path() {
        "/realms/master/login-actions/authenticate" => {
            if let Some(query) = uri.query() {
                let params: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes()).map(|(k, v)| (k.to_string(), v.to_string())).collect();
                    // Checks all query parameters

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "tab_id") {
                        tab_id = value.to_string();
                    }

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "client_id") {
                        if value != "security-admin-console" {
                            return Ok(("/err/login".to_string(), tab_id.to_string()));
                        }
                    }else{
                        return Ok(("/err/login".to_string(), tab_id.to_string()));
                    }

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "tab_id") {
                        if value.len() != 11 {
                            return Ok(("/err/login/tabid".to_string(), tab_id.to_string()));
                        }
                    }else{
                        return Ok(("/err/login/tabid".to_string(), tab_id.to_string()));
                    }

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "execution") {
                        if value != "4adb81d3-5be0-43cd-aedf-af7e6d299095" {
                            return Ok(("/err/login/exec".to_string(), tab_id.to_string()));
                        }
                    }
            }
        },

        _ => {}
    }

    Ok((uri.path().to_string(), tab_id.to_string()))
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let mut auth = false;
    let mut username: String = "".to_string();
    let mut path = uri.clone().path().to_string();
    let mut tab_id = "".to_string();

    // let user_agent = req.headers().get(hyper::header::USER_AGENT).map(|value| value.to_str().unwrap_or(""));
    // if let Some(user_agent) = user_agent {
    //     println!("\x1B[1;93m[🦈] User-Agent: {}\x1B[0m\n", user_agent);
    // }
    

    match (&method, uri.path()) {
        (&Method::POST, "/realms/master/login-actions/authenticate") => {
            let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body_str = String::from_utf8(full_body.to_vec()).unwrap();
            auth = true;

            draw_notif(Notif::RPost, &format!("{} : {}", uri.path(), body_str));

            if let Ok(form_data) = from_str::<LoginForm>(&body_str) {
                username = form_data.username.clone();
                println!("\n\x1B[1;31m[😈] Attempting to connect:\x1B[0m");
                println!("  \x1B[35m> Username: {}\x1B[0m", form_data.username);
                println!("  \x1B[35m> Password: {}\x1B[0m\n", form_data.password);
            } else {
                draw_notif(Notif::Err, &format!("({}) Failed to deserialize form data.", uri.path()));
            }

            match check_query_params( &uri ) {
                Ok(( _path, _tab_id )) => {
                    tab_id = _tab_id;
                }
                Err( _error_message ) => {}
            }

            path = "/realms/master/login-actions/authenticate/err".to_string();
        },

        (&Method::GET, "/realms/master/login-actions/authenticate") => {
            auth = true;

            if let Some(query) = uri.query() {
                draw_notif(Notif::RGet, &format!("{}?{}", uri.path(), query));
            } else {
                draw_notif(Notif::RGet, uri.path());
            }

            match check_query_params( &uri ) {
                Ok(( _path, _tab_id )) => {
                    path = _path;
                    tab_id = _tab_id;
                }
                Err( _error_message ) => {}
            }
        },

        (&Method::GET, _) => {
            if let Some(query) = uri.query() {
                draw_notif(Notif::RGet, &format!("{}?{}", uri.path(), query));
            } else {
                draw_notif(Notif::RGet, uri.path());
            }
        },

        (&Method::POST, _) => {
            let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body_str = String::from_utf8(full_body.to_vec()).unwrap();

            draw_notif(Notif::RPost, &format!("{} : {}", uri.path(), body_str));
        },

        _ => {}

    }

    let response_resource = get_resource(path).await?;
    let status_code = response_resource.status();
    let response_headers = response_resource.headers().clone();
    let mut body = response_resource.into_body();

    if auth {
        let full_body = hyper::body::to_bytes(body).await.unwrap();
        let body_str = String::from_utf8(full_body.to_vec()).unwrap();

        let body_str = body_str.replace("{USERNAME}", &xss_filter(&username));
        let body_str = body_str.replace("{TAB_ID}", &tab_id);

        let full_body = Body::from(body_str.into_bytes());

        body = full_body;
    }

    let mut response = Response::builder()
    .status( status_code )
    .body( body )
    .unwrap();

    if response_headers.contains_key("content-type") && response_headers.get("content-type").unwrap() == "text/html" {
        set_attributs_header(&mut response, get_page_id( uri.path().to_string() ));
    }
          
    // Ajouter les headers de la réponse
    for (name, value) in response_headers.iter() {
        if name == "content-length" || name == "date" || name == "Content-Type" {
            continue;
        }

        let cloned_value = value.clone();
        response.headers_mut().insert(name, cloned_value);
    }

    Ok( response )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut clients: Vec<ClientInfo> = Vec::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], WEB_PORT));
    let listener = TcpListener::bind(addr).await?;

    println!("\n\x1B[32m---------------------------------------------\x1B[0m");
    println!("\x1B[1;32m####### 🐝 WasmPot2 Wasi Server 🐝 #######\x1B[0m\n");
    println!("\x1B[32m[📡] Listening on http://{:?}/\x1B[0m", addr);
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

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(echo)).await {
                draw_notif(Notif::Err, &format!("Error serving connection: {}.", err));
            }
        });
    }
}