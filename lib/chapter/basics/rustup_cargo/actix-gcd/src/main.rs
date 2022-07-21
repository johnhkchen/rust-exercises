use actix_web::{get, App, HttpServer, http::header::ContentType, HttpResponse, Responder};

#[get("/")]
async fn greet() -> impl Responder {
    let body = "data"
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}