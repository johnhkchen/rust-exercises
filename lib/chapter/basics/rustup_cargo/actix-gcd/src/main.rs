use actix_web::{middleware, web, App, HttpRequest, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// fn get_index() -> HttpResponse {
//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(
//             r#"
//                 <title>GCD Calculator</title>
//                 <form action="/gcd" method="post">
//                     <input type="text" name="n" />
//                     <input type="text" name="m" />
//                     <button type="submit">Compute GCD</button>
//                 </form>
//             "#,
//         )
// }

#[cfg(test)]
mod tests {
    use actix_web::body::to_bytes;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    use super::*;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

        Ok(())
    }
}