use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use async_std::fs;
use actix_files::Files;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

static WEB_PORT: u16 = 8888;
static WEB_WASI_PORT: u16 = 8000;

fn generate_random_string(len: usize) -> String {
    let rng = thread_rng();
    let random_string: String = rng.sample_iter(&Alphanumeric).take(len).map(char::from).collect();
    random_string
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
    let random_tab_id = generate_random_string(11);

    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());
    html_content = html_content.replace("{TAB_ID}", &random_tab_id.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/login-actions/authenticate")]
async fn authenticate() -> Result<HttpResponse> {
    let path = "./src/public/authenticate.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/login-actions/authenticate/err")]
async fn authenticate_err() -> Result<HttpResponse> {
    let path = "./src/public/authenticate_err.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());

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

#[get("/err/login")]
async fn err_login() -> Result<HttpResponse> {
    let path = "./src/public/404.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());
    html_content = html_content.replace("{ERR_MESSAGE}", "An error occurred, please login again through your application.");

    let mut response = HttpResponse::NotFound();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/err/login/exec")]
async fn err_login_exec() -> Result<HttpResponse> {
    let path = "./src/public/404_exec.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());

    let mut response = HttpResponse::NotFound();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/err/login/tabid")]
async fn err_login_tabid() -> HttpResponse {
    let mut response = HttpResponse::Found();
    let random_tab_id = generate_random_string(11);

    response.insert_header(("Location", format!("http://localhost:{}/realms/master/login-actions/authenticate?client_id=security-admin-console&tab_id={}", &WEB_WASI_PORT.to_string(), random_tab_id.to_string())));
    response.insert_header(("content-type", "text/html"));
   
    response.finish()
}

async fn not_found() -> Result<HttpResponse, std::io::Error> {
    let path = "./src/public/404.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &WEB_WASI_PORT.to_string());
    html_content = html_content.replace("{ERR_MESSAGE}", "Page not found");

    let mut response = HttpResponse::NotFound();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

/* ############## MAIN FUNCTION ############## */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\n\x1B[32m---------------------------------------------\x1B[0m");
    println!("\x1B[1;32m####### ✨ WasmPot2 Resource Server ✨ #######\x1B[0m\n");
    println!("\x1B[32m[📡] Listening on http://localhost:{}/\x1B[0m", WEB_PORT);
    println!("\x1B[32m[✅] Running !\x1B[0m");
    println!("\x1B[32m---------------------------------------------\x1B[0m\n\n");

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
            .service(err_login)
            .service(err_login_exec)
            .service(err_login_tabid)
            .service(authenticate_err)
            .default_service(
                web::route().to(not_found)
            )
    })
    .bind(("localhost", WEB_PORT))?
    .run()
    .await
}