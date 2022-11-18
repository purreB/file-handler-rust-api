use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn upload_file() -> impl Responder {
    HttpResponse::Ok().body("Upload file route here!")
}

async fn download_file() -> impl Responder {
    HttpResponse::Ok().body("Download file route here!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(upload_file)
            .service(download_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
