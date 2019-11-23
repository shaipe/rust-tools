#![allow(dead_code)] // 该属性用于隐藏对未使用代码的警告

use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::{Duration, Instant};
mod config;
use config::Config;


fn main() {
    // work(100);
    let cnf = Config::new("");
    println!("{:?}", cnf);
    // work(1000);
}

fn work(interval: u64){
    let task = Interval::new(Instant::now(), Duration::from_millis(interval))
        // .take(10)
        .for_each(|instant| {
            println!("fire; instant={:?}", instant);
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
}