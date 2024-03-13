
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "/app/www"))
	    
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
