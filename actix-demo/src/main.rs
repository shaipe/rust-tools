use actix_web::{middleware, web, App, error,
    Error, HttpRequest, HttpServer, HttpResponse};
use std::thread;
use bytes::Bytes;
use futures::{future::ok, Future, Stream};
use futures::unsync::mpsc;

fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    no_loop();
    "Index Hello world!"
}

fn no_loop(){
    thread::spawn(move || {
        let mut x = 1;
        while x <= 10 {
            thread::sleep(std::time::Duration::from_secs(2));
            println!("{:?}", x);
            x += 1;
        }
    });
}

/// async handler
fn index_async(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("{:?}", req);
    no_loop();
    ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Hello {}!", req.match_info().get("name").unwrap())))
}

/// async body
fn index_async_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::unbounded();
    let _ = tx.unbounded_send(Bytes::from(text.as_bytes()));
    // 同步处理
    let mut x = 1;
    while x <= 3 {
        thread::sleep(std::time::Duration::from_secs(1));
        println!("{:?}", x);
        x += 1;
    }
    HttpResponse::Ok()
        .streaming(rx_body.map_err(|_| error::ErrorBadRequest("bad request")))
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| "Hello world!"))
            .service(web::resource("/").to(index))
            // async handler
            .service(
                web::resource("/async/{name}").route(web::get().to_async(index_async)),
            )
            // async handler
            .service(
                web::resource("/async-body/{name}")
                    .route(web::get().to(index_async_body)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}