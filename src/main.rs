use ntex::web;

mod handler;


#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            // Allow all origins, methods, request headers and exposed headers allowed. Credentials supported. Max age 1 hour. Does not send wildcard.
            .wrap(
                ntex_cors::Cors::new() // <- Construct CORS middleware builder
                  // By default All origins are allowed !
                  .max_age(3600)
                  .finish()
            )
            // .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
            .service(web::scope("/api")
                .configure(handler::configure)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}