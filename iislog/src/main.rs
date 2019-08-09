/// 对iis日志进行分析处理,去除指定的ip段请求

use std::env;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::io::prelude::*;

fn main() {
    let args : Vec<String> = env::args().collect();

    // 源目录
    let file_path = if args.len() > 1 {
        &args[1]
    }
    else{
        panic!("please input file path!");
    };

    // 目标文件
    let dist_path = if args.len() > 2 {
        &args[2]
    }
    else{
        "result.log"
    };

    // 处理类型,默认为单文件处理
    let deal_type = if args.len() > 3 {
        &args[3]
    }
    else{
        "file"
    }

    

    println!("{}", file_path);
}

fn deal_dir(src_dir: &str, dist_dir: &str) {
    
}

fn deal_file(file_path: &str, dist_path: &str){
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => panic!("error {:?}", e)
    };

    let mut dist_file = match File::create(dist_path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let re = Regex::new("^.*(100\\.116\\.\\d{1,3}\\.\\d{1,3}).*$").unwrap();

    for line in reader.lines() {
        let line: String = line.unwrap();
        if !re.is_match(&line) {
            match dist_file.write_all(format!("{}\n", line).as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to : {}", why)
                },
                Ok(_) => {println!("successfully wrote to {}", line)},
            };
        }
        else{
            println!("{}", line);
        }
       
    }
}