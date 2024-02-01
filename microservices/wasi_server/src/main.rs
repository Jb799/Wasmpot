use std::net::SocketAddr;
use std::env;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Response, Uri};
use hyper::header::HeaderValue;

use tokio::net::TcpListener;

mod functions;
use functions::set_attributs_header;
use functions::get_page_id;

use serde::Deserialize;
use serde_urlencoded::from_str;
use serde_urlencoded::to_string;

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

fn tabid_filter(input: &str) -> String {
    let mut filtered = String::new();

    for c in input.chars() {
        if c.is_alphanumeric() || c == '_' {
            filtered.push(c);
        }
    }

    filtered
}

fn check_query_params( uri: &Uri ) -> Result<(String, String), String> {
    let mut tab_id = "".to_string();
    let mut state = "".to_string();

    match uri.path() {
        "/realms/master/protocol/openid-connect/auth" =>{
            if let Some(query) = uri.query() {
                let params: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes()).map(|(k, v)| (k.to_string(), v.to_string())).collect();
                    // Checks all query parameters
                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "state") {
                        state = value.to_string();
                        /* 
                            State: 36 characters | Token anti CSRF
                        */
                    }

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "code_challenge_method") {
                        if value != "S256" {
                            return Ok(("/err/auth/ccmethod_invalid".to_string(), state.to_string()));
                        }
                    }else{
                        return Ok(("/err/auth/ccmethod_nfound".to_string(), state.to_string()));
                    }

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "code_challenge") {
                        if value.len() != 43 {
                            return Ok(("/err/auth/cc_invalid".to_string(), state.to_string()));
                        }
                    }else{
                        return Ok(("/err/auth/cc_nfound".to_string(), state.to_string()));
                    }

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "client_id") {
                        if value != "security-admin-console" {
                            return Ok(("/err/bad_client".to_string(), "".to_string()));
                        }
                    }else{
                        return Ok(("/err/bad_client".to_string(), "".to_string()));
                    }

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "response_mode") {
                        if value != "query" && value != "fragment" && value != "form_post" && value != "query.jwt" && value != "fragment.jwt" && value != "form_post.jwt" && value != "jwt" {
                            return Ok(("/err/auth/rm_invalid".to_string(), state.to_string()));
                        }
                    }

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "response_type") {
                        if value != "code" && value != "none" {
                            return Ok(("/err/auth/rt_invalid".to_string(), state.to_string()));
                        }
                    }else{
                        return Ok(("/err/auth/rt_nfound".to_string(), state.to_string()));
                    }

                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "scope") {
                        if value != "openid" && value != "acr" && value != "email" && value != "phone" && value != "roles" && value != "address" && value != "web-origins" && value != "microprofile-jwt" && value != "offline_access" && value != "profile"{
                            return Ok(("/err/auth/scope_invalid".to_string(), state.to_string()));
                        }
                    }
            }
        },

        "/realms/master/login-actions/authenticate" => {
            if let Some(query) = uri.query() {
                let params: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes()).map(|(k, v)| (k.to_string(), v.to_string())).collect();
                    // Checks all query parameters
                    if let Some((_key, value)) = params.iter().find(|(_key, _)| _key == "tab_id") {
                        tab_id = value.to_string();
                        /* 
                            Tab_Id: 11 characters | identifier of the tab browser
                        */
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

async fn echo(req: Request<Body>, web_res_port: u16, web_res_name: String) -> Result<Response<Body>, hyper::Error> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let mut auth = false;
    let mut state_redirect = false;
    let mut username: String = "".to_string();
    let mut path = uri.clone().path().to_string();
    let mut tab_id = "".to_string();
    let mut state = "".to_string();

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

        (&Method::GET, "/realms/master/protocol/openid-connect/auth") => {
            if let Some(query) = uri.query() {
                draw_notif(Notif::RGet, &format!("{}?{}", uri.path(), query));
            } else {
                draw_notif(Notif::RGet, uri.path());
            }

            match check_query_params( &uri ) {
                Ok(( _path, _state )) => {
                    path = _path;
                    if _state.len() > 0 {
                        state = _state;
                        state_redirect = true;
                    }
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

    // Tab_ID filter
    let tab_id_filtered = tabid_filter(&tab_id.clone());
    if auth && tab_id_filtered.len() != 11{
        path = "/err/login/tabid".to_string();
        auth = false;
        println!("\n\x1B[1;31m[😈] Probably tried to give an injection:\x1B[0m");
        println!("  \x1B[35m> GET (tab_id): {}\x1B[0m\n", tab_id.to_string());
    }

    // State filter
    if state.len() > 0 && state.len() != 36{
        println!("\n\x1B[1;31m[😈] Probably tried to give an injection:\x1B[0m");
        println!("  \x1B[35m> GET (state): {}\x1B[0m\n", state.to_string());
    }
    state = to_string(&[("state", state)]).unwrap();

    let response_resource = get_resource(path.clone(), web_res_port, web_res_name).await?;
    let status_code = response_resource.status();
    let response_headers = response_resource.headers().clone();
    let mut body = response_resource.into_body();

    if auth {
        let full_body = hyper::body::to_bytes(body).await.unwrap();
        let body_str = String::from_utf8(full_body.to_vec()).unwrap();

        let body_str = body_str.replace("{USERNAME}", &xss_filter(&username));
        let body_str = body_str.replace("{TAB_ID}", &tab_id_filtered);

        let full_body = Body::from(body_str.into_bytes());

        body = full_body;
    }

    let mut response = Response::builder()
    .status( status_code )
    .body( body )
    .unwrap();

    if response_headers.contains_key("content-type") && response_headers.get("content-type").unwrap() == "text/html" {
        set_attributs_header(&mut response, get_page_id( path ));
    }

    // Ajouter les headers de la réponse
    for (name, value) in response_headers.iter() {
        if name == "Content-length" || name == "Date" || name == "Content-Type" {
            continue;
        }

        let cloned_value = value.clone();

        if name == "Location" {
            if let Ok(mut location_value) = value.to_str(){
                let new_location_value = location_value.replace("[STATE]", &state);
                let new_value = HeaderValue::from_str(&new_location_value).unwrap();
                response.headers_mut().insert(name, new_value);
            }else{
                response.headers_mut().insert(name, cloned_value);
            }
        } else {
            response.headers_mut().insert(name, cloned_value);
        }
    }

    Ok( response )
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