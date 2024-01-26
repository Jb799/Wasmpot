use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Response, Uri};

use tokio::net::TcpListener;

mod functions;
use functions::set_attributs_header;

static WEB_PORT: u16 = 8080;
static WEB_RESOURCES_PORT: u16 = 8888;

async fn get_resource(uri: Uri) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    let uri = format!("http://localhost:{}{}", WEB_RESOURCES_PORT, uri.path());
    println!("uri: {}", uri);

    let uri = uri.parse::<Uri>().unwrap();

    let res = client.get(uri).await?;

    Ok(res)
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/realms/master/login-actions/authenticate") => {
            println!("POST LOGIN /{}", req.uri().path());
        },

        (&Method::GET, _) => {
            // println!("GET /{}", req.uri().path());
        },

        (&Method::POST, _) => {
            // println!("POST /{}", req.uri().path());
        },

        _ => {
            println!("404");
        }

    }

    let response_resource = get_resource(req.uri().path().parse().unwrap()).await?;
    let status_code = response_resource.status();
    let response_headers = response_resource.headers().clone();

    let mut response = Response::builder()
    .status( status_code )
    .body( response_resource.into_body() )
    .unwrap();

    if response_headers.contains_key("content-type") && response_headers.get("content-type").unwrap() == "text/html" {
        set_attributs_header(&mut response, status_code.as_u16());
    }

    // Ajouter les headers de la réponse
    for (name, value) in response_headers.iter() {
        if name == "content-length" || name == "date" || name == "Content-Type" {
            continue;
        }
        
        println!("Header - {}: {}", name, value.to_str().unwrap());

        let cloned_value = value.clone();
        response.headers_mut().insert(name, cloned_value);
    }

    Ok( response )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], WEB_PORT));
    let listener = TcpListener::bind(addr).await?;

    println!("------------------------------------");
    println!("####### 🐝 WasmPot2 Wasi Server 🐝 #######\n");
    println!("[📡] Listening on http://{}/", addr);
    println!("[✅] Running !");
    println!("------------------------------------");

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(echo)).await {
                println!("[🚨] Error serving connection: {:?}", err);
            }
        });
    }
}