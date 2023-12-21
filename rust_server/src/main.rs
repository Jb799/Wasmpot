use actix_web::{get, web, App, HttpResponse, HttpServer};
mod functions;
use functions::set_attributs_header;
use actix_files::NamedFile;

/* ############## API DEFINE ############## */
#[get("/")]
async fn index() -> HttpResponse {
    let path = "./src/public/index.html";
    let _file = NamedFile::open(path);

    let mut response = HttpResponse::Ok();

    set_attributs_header(&mut response, 200);

    response.body("<p>INCLURE FICHIER HTML</p>")
}

#[get("/admin")]
async fn admin() -> HttpResponse {
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
            .service(index)
            .service(admin)
            .service(admin_master_console)
            .default_service(
                web::route().to(not_found)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}