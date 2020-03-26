use actix_web::{web, Error, HttpRequest, HttpResponse};
use bytes::BytesMut;
use chrono::Local;
use futures::StreamExt;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::thread;

pub async fn collect_post(
    req: HttpRequest,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let site = req.query_string().to_string();
    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        // if (body.len() + chunk.len()) > MAX_SIZE {
        //     return Err(error::ErrorBadRequest("overflow"));
        // }
        body.extend_from_slice(&chunk);
    }

    // 异步写入数据
    thread::spawn(move || {
        let file_path = format!("errors/{}_{}.log", site, Local::today().format("%Y%m%d"));
        append_content(Path::new(&file_path), &body);
    });

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("{}"))
}

/// 追加文件
fn append_content(file_path: &Path, content: &[u8]) {
    // 以读,写,创建,追加的方式打开文件
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_path);

    // 向文件中写入内容
    match file {
        Ok(mut stream) => {
            // if stream.len() > 1024 * 2 {
            //     append_content(file_path, content);
            // }
            // else{
            stream.write_all(content).unwrap();
            stream.write_all(",\n".as_bytes()).unwrap();
            // }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
