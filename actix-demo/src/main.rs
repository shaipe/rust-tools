#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_files as fs;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use bytes::Bytes;



/// simple index handler
#[get("/")]
async fn welcome(req: HttpRequest) -> Result<HttpResponse> {
    let q = "google";
    let request_uri = format!("https://www.baidu.com/s?wd={}", q);

    Ok(HttpResponse::TemporaryRedirect()
        .header(header::LOCATION, request_uri)
        .finish()
    }
)




#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(welcome)
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
