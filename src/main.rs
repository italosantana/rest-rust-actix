use actix_web::*;

mod routes;
use routes::ping::*;
use routes::info::*;
use routes::catalogo::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(ping))
        .route("/info", web::get().to(info))
        .route("/cat", web::get().to(catalogo))
    });

    let port = 9091;

    let api = api.bind (format!("127.0.0.1:{}", port))
    .expect("Could not connect...");
    
    println!("Connect with success... \n http://localhost:{}/ping", port);

    api.run()
    .await
}