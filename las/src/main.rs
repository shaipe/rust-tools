// copyright © shaipe 2020
//! sst server

use actix_web::{middleware, web, App, HttpServer};
mod collect;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 1. 初始化配置
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=trace");
    env_logger::init();

    let addr = format!("{}:{}", "0.0.0.0", 8089);

    println!("site {} started", addr);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("X-Version", "0.1")
                    .header("sever", "las-server")
            )
            .service(
                web::resource("/collect")
                    .route(web::post().to(collect::collect_post)),
            )
    })
    .bind(addr)?
    // .workers(1) // 指定启用的工作线程数
    .run()
    .await
}
