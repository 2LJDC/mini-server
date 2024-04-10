
use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use actix_web::HttpRequest;
//use actix_web::{web, http::header};
use actix_web::middleware;


//use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::Error;

use jason_to_postgres::del_customer;
use jason_to_postgres::add_customer;
use jason_to_postgres::dump_database;

// login test
#[get("/test")]
async fn test(request: HttpRequest) -> impl Responder {
	let req_headers = request.headers();
	let basic_auth_header = req_headers.get("Authorization");
	let basic_auth: &str = basic_auth_header.unwrap().to_str().unwrap();
	println!("{}", basic_auth);

	if "Bearer eyJ1c2VybmFtZSI6Imtla3ciLCJwYXNzd29yZCI6Im5vcGUifQ==" != basic_auth {
		return HttpResponse::Ok().body("false");
	}

	HttpResponse::Ok().body("true")
}

// database
async fn printdata(request: HttpRequest) -> impl Responder {
	let req_headers = request.headers();
	let basic_auth_header = req_headers.get("Authorization");
	let basic_auth: &str = basic_auth_header.unwrap().to_str().unwrap();

	if "Bearer eyJ1c2VybmFtZSI6Imtla3ciLCJwYXNzd29yZCI6Im5vcGUifQ==" != basic_auth {
		return HttpResponse::Ok().body("false");
	}

	let url = format!("postgres://postgres:{}@{}:{}", "deeznuts", "85.215.154.152", "5432");
	let data = dump_database(&url).await.unwrap();
	
	HttpResponse::Ok().body(data)
}


	

// status
async fn status() -> String {
    "Server is up and running.".to_string()
}


// index
async fn index() -> Result<fs::NamedFile, Error> {
    let file = fs::NamedFile::open("/app/www/index.html")?;
    Ok(file)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
		.wrap(middleware::Compress::default())
		.route("/status", web::get().to(status))
		.route("/", web::get().to(index))
		.route("/login", web::get().to(index))
		.route("/uploader", web::get().to(index))
		.route("/printdata", web::get().to(printdata))
	    .service(test)
        .service(fs::Files::new("/", "/app/www"))
		.default_service(web::get().to(index))
	    
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}




