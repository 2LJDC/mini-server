
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use actix_web::HttpRequest;
use actix_web::{web, http::header};

// login
//#[get("/login")]
/*async fn login(req: &HttpRequest) -> impl Responder {
	async fn index(date: web::Header<header::Date>) -> String {
	let txt = format!("Request was sent at {}", date.to_string())
}

    HttpResponse::Ok()
}*/

#[get("/test")]
async fn test(date: web::Header<header::Date>) -> impl Responder {
	let txt = format!("Request was sent at {}", date.to_string());
	println!("");
	HttpResponse::Ok()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
	    .service(login)
            .service(fs::Files::new("/", "/app/www"))
	    
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
