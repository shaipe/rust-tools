/// 对iis日志进行分析处理,去除指定的ip段请求

use std::env;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::thread;

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
    };

    // 对数据开始处理
    if deal_type == "file" {
        deal_file(&file_path, &dist_path);
    }
    else{
        deal_dir(&file_path, &dist_path, true);
    }
    
    println!("{}", file_path);
}

/// 对目录进行处理
fn deal_dir(src_dir: &str, dist_dir: &str, is_mutli_thread: bool) {
     
     // // 提供一个 vector 来存放所创建的子线程（children）。
    let mut children = vec![];
     
     for entry in read_dir(src_dir).unwrap(){
        let p: PathBuf = entry.unwrap().path();
        if p.is_file() {
            let ext = p.extension();
            if !ext.is_none() {
                if ext.unwrap().to_str().unwrap().to_lowercase() == "log" {
                    let dist_file = format!("{:?}/{:?}", dist_dir.to_owned() , p.clone().file_name());
                    
                    if is_mutli_thread {
                        
                        // // 启用多线程的方式进行文件分析
                        // // 启动（spin up）另一个线程
                        children.push(thread::spawn( move || {
                            let fi_path = p.to_str().unwrap();
                            // let my = self.clone();
                            deal_file(&fi_path, &dist_file);
                        }));
                    }
                    else{
                        let fi_path = p.to_str().unwrap();
                        // // println!("开始分析文件:{:?}.", f_path);
                        deal_file(&fi_path, &dist_file);
                    }
                }
            }
        }
     }

     if is_mutli_thread {
        // 等待线程结束
        for child in children {
            // 等待线程结束。返回一个结果。
            let _ = child.join();
        }
    }
}

/// 对单个文件进行处理
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