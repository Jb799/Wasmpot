use actix_web::{get, post, web, App, HttpResponse, HttpServer, Result};
use async_std::fs;
use actix_files::Files;
use serde::Deserialize;
use html_escape::encode_double_quoted_attribute;

static WEB_PORT: u16 = 8888;
static WEB_WASI_PORT: u16 = 8080;

#[derive(Deserialize)]
struct LoginFormInfo {
    username: String,
}

/* ############## API DEFINE ############## */
#[get("/")]
async fn index() -> Result<HttpResponse> {
    let path = "./src/public/index.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());
    let mut response = HttpResponse::Ok();

    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/admin/")]
async fn admin_redirect() -> HttpResponse {
    let mut response = HttpResponse::Found();
    response.insert_header(("Location", "/admin/master/console/"));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/admin/master/console/")]
async fn admin_master_console() -> Result<HttpResponse> {
    let path = "./src/public/admin.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content)) 
}

#[get("/realms/master/protocol/openid-connect/auth")]
async fn auth() -> Result<HttpResponse> {
    let path = "./src/public/auth.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[post("/realms/master/login-actions/authenticate")]
async fn authenticate(form: web::Form<LoginFormInfo>) -> Result<HttpResponse> {
    let path = "./src/public/authenticate.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());

    if !form.username.is_empty() {
        // Échapper l'entrée utilisateur pour éviter les attaques XSS
        let escaped_username = encode_double_quoted_attribute(&form.username);
        html_content = html_content.replace("{USERNAME}", &escaped_username);
    }else{
        html_content = html_content.replace("{USERNAME}", "");
    }

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/protocol/openid-connect/3p-cookies/step2.html")]
async fn step2() -> Result<HttpResponse> {
    let path = "./src/public/step2.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("P3p", "CP=\"This is not a P3P policy!\""));
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/protocol/openid-connect/3p-cookies/step1.html")]
async fn step1() -> Result<HttpResponse> {
    let path = "./src/public/step1.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("P3p", "CP=\"This is not a P3P policy!\""));
    response.insert_header(("Cache-Control", "no-cache"));
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/admin/master/console/config")]
async fn config() -> HttpResponse {
    let mut response = HttpResponse::Ok();
    response.insert_header(("Content-Type", "application/json"));
    response.insert_header(("content-type", "text/html"));

    // Config json:
    let mut json = String::from("{");

    json.push_str("\"realm\": \"master\",");

    let auth_server_url = format!("\"auth-server-url\": \"http://localhost:{}/\",", WEB_WASI_PORT);
    json.push_str(&auth_server_url);

    json.push_str("\"ssl-required\": \"external\",");
    json.push_str("\"resource\": \"security-admin-console\",");
    json.push_str("\"public-client\": true,");
    json.push_str("\"confidential-port\": 0");
    json.push_str("}");

    response.body(json)
}

async fn not_found() -> Result<HttpResponse, std::io::Error> {
    let path = "./src/public/404.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());

    let mut response = HttpResponse::NotFound();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

/* ############## MAIN FUNCTION ############## */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("------------------------------------");
    println!("####### ✨ WasmPot2 Resource Server ✨ #######\n");
    println!("[📡] Listening on http://localhost:{}/", WEB_PORT);
    println!("[✅] Running !");
    println!("------------------------------------");

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/resources", "./src/public/resources"))
            .service(Files::new("/welcome-content", "./src/public/welcome-content"))
            .service(Files::new("/js", "./src/public/js"))
            .service(index)
            .service(admin_redirect)
            .service(admin_master_console)
            .service(config)
            .service(auth)
            .service(step1)
            .service(step2)
            .service(authenticate)
            .default_service(
                web::route().to(not_found)
            )
    })
    .bind(("localhost", WEB_PORT))?
    .run()
    .await
}