
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use actix_web::HttpRequest;
use actix_web::{web, http::header};
use actix_web::middleware;


use actix_multipart::Multipart;
use actix_web::Error;
use futures_util::StreamExt as _;
use futures_util::stream::stream::StreamExt;
    |

// login
//#[get("/login")]
/*async fn login(req: &HttpRequest) -> impl Responder {
	async fn index(date: web::Header<header::Date>) -> String {
	let txt = format!("Request was sent at {}", date.to_string())
}

    HttpResponse::Ok()
}*/

//#[get("/test")]
//async fn test(date: web::Header<header::Date>) -> impl Responder {
	//let txt = format!("Request was sent at {}", date.to_string());

#[get("/test")]
async fn test(request: HttpRequest) -> impl Responder {
	let req_headers = request.headers();
	let basic_auth_header = req_headers.get("Authorization");
	let basic_auth: &str = basic_auth_header.unwrap().to_str().unwrap();
	println!("{}", basic_auth);
	//let data = json::parse(&format!("{:?}", HttpRequest::headers(&req))).unwrap();
	//let data = req.headers().get("username").unwrap().to_str().ok();
	//if let Some(user) = req.headers().get("username").unwrap().to_str().ok(){
	//	println!("{}", user);
	//} else {
	//	println!("nope");
	//}

	//println!("username: {}", data);
	//println!("password: {}", data["password"]);
	//println!("authorization: {}", data["authorization"]);
	//println!("{:?}", req);
	HttpResponse::Ok().body("true")
}

// index
#[get("/")]
async fn index() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/app/www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}


// index
#[get("/uploader")]
async fn uploader() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/app/www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}
// index
#[get("/login")]
async fn login() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/app/www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}


async fn index2() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/app/www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}



async fn index(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
           let mut field = item?;

           // Field in turn is stream of *Bytes* object
           while let Some(chunk) = field.next().await {
               println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
           }
    }

    Ok(HttpResponse::Ok().into())
}







#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
		.wrap(middleware::Compress::default())
	    .service(test)
		.service(index)
		.service(uploader)
		.service(login)
        .service(fs::Files::new("/", "/app/www"))
		.service(web::resource("/{project_id}")
			.route(web::put().to(|| HttpResponse::Ok())))
		.default_service(web::get().to(index2))
	    
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
