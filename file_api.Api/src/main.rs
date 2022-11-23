//TODO: Add Controllers with actix web, make seperate [file]Controller for each filetype
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

extern crate file_api_core;
extern crate file_api_persistence;

#[post("/upload-file")]
async fn upload_file() -> impl Responder {
    HttpResponse::Ok().body("Upload file route here!")
}

#[get("/download-file")]
async fn download_file() -> impl Responder {
    HttpResponse::Ok().body("Download file route here!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(upload_file)
                .service(download_file),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
