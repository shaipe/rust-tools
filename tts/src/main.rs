#![allow(dead_code)] // 该属性用于隐藏对未使用代码的警告

use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::{Duration, Instant};

fn main() {
    work(100);
    work(200);
}

fn work(interval: u64){
    let task = Interval::new(Instant::now(), Duration::from_millis(interval))
        .take(10)
        .for_each(|instant| {
            println!("fire; instant={:?}", instant);
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
}