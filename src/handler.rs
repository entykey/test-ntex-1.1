use ntex::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1.1")
            .route("", web::get().to(hello))
            .route("/echo", web::post().to(echo))
            .route("/hey", web::get().to(manual_hello))
    );
}

async fn hello(req: web::HttpRequest) -> web::HttpResponse {
    
    // DOCS: pub fn peer_addr(&self) -> Option<SocketAddr>
    if let Some(val) = req.peer_addr() {
        println!("accepted a connection from {:?}", val.ip());
    };

    web::HttpResponse::Ok().body("Ntex 1.1 is working properly")
}

async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hey there!")
}