
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use actix_web::HttpRequest;


// login
#[get("/login")]
async fn login(req: &HttpRequest) -> impl Responder {
	let txt = req.headers().get("content-type").expect("nope").to_str().ok();
	println!("{:?}", txt);

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
