//TODO: Add Controllers with actix web, make seperate [file]Controller for each filetype
use actix_web::{
    get, post,
    web::{self, service},
    App, HttpResponse, HttpServer, Responder,
};
use file_api_core::module_domain::models::Pdf;
use uuid::Uuid;

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

#[get("/test")]
async fn test() -> impl Responder {
    let test_pdf: Pdf = Pdf::new(
        Uuid::new_v4(),
        "test".to_string(),
        1337,
        vec![1, 2, 3],
        "Test".to_string(),
    );
    println!("{:?}", test_pdf);

    HttpResponse::Ok().body("Test Route here!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(upload_file)
                .service(download_file)
                .service(test),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
