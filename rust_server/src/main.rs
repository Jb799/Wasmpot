use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
mod functions;
use functions::set_attributs_header;
use async_std::fs;
use actix_files::Files;

/* ############## API DEFINE ############## */
#[get("/")]
async fn index() -> Result<HttpResponse> {
    let path = "./src/public/index.html";
    let html_content = fs::read_to_string(path).await?;

    let mut response = HttpResponse::Ok();
    set_attributs_header(&mut response, 200);

    Ok(response.body(html_content))
}

#[get("/admin/")]
async fn admin_redirect() -> HttpResponse {
    let mut response = HttpResponse::Found();
    
    set_attributs_header(&mut response, 300);
    response.insert_header(("Location", "/admin/master/console/"));

    response.finish()
}

#[get("/admin/master/console/")]
async fn admin_master_console() -> HttpResponse {
    let mut response = HttpResponse::Ok();
    
    set_attributs_header(&mut response, 200);
    
    response.body("<p>console master</p>")
}

async fn not_found() -> HttpResponse {
    let mut response = HttpResponse::NotFound();

    set_attributs_header(&mut response, 400);
    
    response.body("<p>404 - Not Found</p>")
}

/* ############## MAIN FUNCTION ############## */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/resources", "./src/public/resources"))
            .service(Files::new("/welcome-content", "./src/public/welcome-content"))
            .service(index)
            .service(admin_redirect)
            .service(admin_master_console)
            .default_service(
                web::route().to(not_found)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}