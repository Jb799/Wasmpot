use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use async_std::fs;
use actix_files::Files;
use std::env;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn generate_random_string(len: usize) -> String {
    let rng = thread_rng();
    let random_string: String = rng.sample_iter(&Alphanumeric).take(len).map(char::from).collect();
    random_string
}

/* ############## API DEFINE ############## */
#[get("/")]
async fn index(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/index.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());
    let mut response = HttpResponse::Ok();

    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/admin/")]
async fn admin_redirect(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();

    response.insert_header(("Location", format!("http://localhost:{}/admin/master/console/", &web_wasi_port.to_string())));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/admin/master/console/")]
async fn admin_master_console(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/admin.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content)) 
}

#[get("/realms/master/protocol/openid-connect/auth")]
async fn auth(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/auth_demo.html"; // DEBUG DEMO: a changer par auth.html
    let mut html_content = fs::read_to_string(path).await?;
    let random_tab_id = generate_random_string(11);

    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());
    html_content = html_content.replace("{TAB_ID}", &random_tab_id.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/login-actions/authenticate")]
async fn authenticate(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/authenticate.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/login-actions/authenticate/err")]
async fn authenticate_err(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/authenticate_err.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/protocol/openid-connect/3p-cookies/step2.html")]
async fn step2(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/step2.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("P3p", "CP=\"This is not a P3P policy!\""));
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/protocol/openid-connect/3p-cookies/step1.html")]
async fn step1(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/step1.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("P3p", "CP=\"This is not a P3P policy!\""));
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/protocol/openid-connect/login-status-iframe.html")]
async fn login_status(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/login_status.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("P3p", "CP=\"This is not a P3P policy!\""));
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/realms/master/.well-known/openid-configuration")]
async fn openid_config(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/openid_config.json";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());

    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/admin/master/console/config")]
async fn config(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/html"));

    // Config json:
    let mut json = String::from("{");

    json.push_str("\"realm\": \"master\",");

    let auth_server_url = format!("\"auth-server-url\": \"http://localhost:{}/\",", &web_wasi_port.to_string());
    json.push_str(&auth_server_url);

    json.push_str("\"ssl-required\": \"external\",");
    json.push_str("\"resource\": \"security-admin-console\",");
    json.push_str("\"public-client\": true,");
    json.push_str("\"confidential-port\": 0");
    json.push_str("}");

    response.body(json)
}

#[get("/err/bad_client")]
async fn err_bad_client(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/404.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());
    html_content = html_content.replace("{ERR_MESSAGE}", "Client not found.");

    let mut response = HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST);
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/err/login")]
async fn err_login(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/404.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());
    html_content = html_content.replace("{ERR_MESSAGE}", "An error occurred, please login again through your application.");

    let mut response = HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST);
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/err/auth/ccmethod_invalid")]
async fn ccmethod_invalid(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();
    response.insert_header(("Location", format!("http://localhost:{}/admin/master/console/#error=invalid_request&error_description=Invalid+parameter%3A+code+challenge+method+is+not+configured+one&[STATE]", &web_wasi_port.to_string())));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/err/auth/ccmethod_nfound")]
async fn ccmethod_nf(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();
    response.insert_header(("Location", format!("http://localhost:{}/admin/master/console/#error=invalid_request&error_description=Missing+parameter%3A+code_challenge_method&[STATE]", &web_wasi_port.to_string())));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/err/auth/cc_invalid")]
async fn cc_invalid(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();
    response.insert_header(("Location", format!("http://localhost:{}/admin/master/console/#error=invalid_request&error_description=Invalid+parameter%3A+code_challenge&[STATE]", &web_wasi_port.to_string())));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/err/auth/cc_nfound")]
async fn cc_nf(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();
    response.insert_header(("Location", format!("http://localhost:{}/admin/master/console/#error=invalid_request&error_description=Missing+parameter%3A+code_challenge&[STATE]", &web_wasi_port.to_string())));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/err/auth/rm_invalid")]
async fn rm_invalid(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();
    response.insert_header(("Location", format!("http://localhost:{}/admin/master/console/?error=invalid_request&error_description=Invalid+parameter%3A+response_mode&[STATE]", &web_wasi_port.to_string())));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/err/auth/rt_invalid")]
async fn rt_invalid(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();
    response.insert_header(("Location", format!("http://localhost:{}/admin/master/console/#error=unauthorized_client&error_description=Client+is+not+allowed+to+initiate+browser+login+with+given+response_type.+Implicit+flow+is+disabled+for+the+client.&[STATE]", &web_wasi_port.to_string())));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/err/auth/rt_nfound")]
async fn rt_nf(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();
    response.insert_header(("Location", format!("http://localhost:{}/admin/master/console/#error=invalid_request&error_description=Missing+parameter%3A+response_type&[STATE]", &web_wasi_port.to_string())));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/err/auth/scope_invalid")]
async fn scope_invalid(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();
    response.insert_header(("Location", format!("http://localhost:{}/admin/master/console/#error=invalid_scope&error_description=Invalid+scopes%3A+opend&[STATE]", &web_wasi_port.to_string())));
    response.insert_header(("content-type", "text/html"));

    response.finish()
}

#[get("/err/login/exec")]
async fn err_login_exec(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse> {
    let path = "./src/public/404_exec.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());

    let mut response = HttpResponse::NotFound();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

#[get("/err/login/tabid")]
async fn err_login_tabid(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> HttpResponse {
    let mut response = HttpResponse::Found();
    let random_tab_id = generate_random_string(11);

    response.insert_header(("Location", format!("http://localhost:{}/realms/master/login-actions/authenticate?client_id=security-admin-console&tab_id={}", &web_wasi_port.to_string(), random_tab_id.to_string())));
    response.insert_header(("content-type", "text/html"));
   
    response.finish()
}

async fn not_found(web_wasi_port: web::Data<u16>, web_wasi_addr: web::Data<String>) -> Result<HttpResponse, std::io::Error> {
    let path = "./src/public/404.html";
    let mut html_content = fs::read_to_string(path).await?;
    html_content = html_content.replace("{WEB_PORT}", &web_wasi_port.to_string());
    html_content = html_content.replace("{WEB_ADDR}", &web_wasi_addr.to_string());
    html_content = html_content.replace("{ERR_MESSAGE}", "Page not found");

    let mut response = HttpResponse::NotFound();
    response.insert_header(("content-type", "text/html"));

    Ok(response.body(html_content))
}

/* ############## MAIN FUNCTION ############## */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let web_port: u16 = env::var("WEB_PORT").unwrap_or("8888".to_string()).parse().unwrap();
    let web_wasi_port: u16 = env::var("WEB_WASI_PORT").unwrap_or("8000".to_string()).parse().unwrap();
    let web_wasi_addr: String = env::var("WEB_ADDR").unwrap_or("localhost".to_string()).parse().unwrap();

    println!("\n\x1B[32m---------------------------------------------\x1B[0m");
    println!("\x1B[1;32m####### ✨ WasmPot2 Resource Server ✨ #######\x1B[0m\n");
    println!("\x1B[32m[📡] Listening on http://localhost:{}/\x1B[0m", web_port);
    println!("\x1B[32m[✅] Running !\x1B[0m");
    println!("\x1B[32m---------------------------------------------\x1B[0m\n\n");

    HttpServer::new(move || {
        App::new()
            .data(web_wasi_port)
            .data(web_wasi_addr.to_string())
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
            .service(ccmethod_invalid)
            .service(ccmethod_nf)
            .service(err_bad_client)
            .service(cc_invalid)
            .service(cc_nf)
            .service(login_status)
            .service(rm_invalid)
            .service(rt_invalid)
            .service(rt_nf)
            .service(scope_invalid)
            .service(openid_config)
            .default_service(
                web::route().to(not_found)
            )
    })
    .bind(("0.0.0.0", web_port))?
    .run()
    .await
}