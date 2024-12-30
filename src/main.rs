use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("hello {}!", &name)
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Start a server backend, listen:localhost:8000");
    HttpServer::new(|| {
        App::new()
            // use get protocol
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    // support string name
    .bind("localhost:8000")?
    .run()
    .await
}
