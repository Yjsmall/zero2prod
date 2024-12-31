use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("hello {}!\n", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Start a server backend, listen:localhost:8000");
    HttpServer::new(|| {
        App::new()
            // use get protocol
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(health_check))
    })
    // support string name
    .bind("localhost:8000")?
    .run()
    .await
}
