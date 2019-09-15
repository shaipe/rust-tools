// #[macro_use]
// extern crate log;
// extern crate log4rs;
/**
 * 过滤文件类型
 */

use std::path::Path;
use std::env;
use std::fs::{read_dir, File};
use std::io::prelude::*;
// use log4rs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let src_dir = if args.len() > 1 {
        &args[1]
    }
    else {
        "./"
    };

    let file_ext = if args.len() > 2 {
        &args[2]
    }
    else {
        "md"
    };

    // 日志系统初始化
    // log4rs::init_file("log.yaml", Default::default()).unwrap();
    let write_path = format!("{}/{}.csv", &src_dir, file_ext);
    println!("{}", write_path);
    let mut file = match File::create(&write_path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    walk_dir(Path::new(src_dir), file_ext, &mut file);

}

/// 遍历给定的目录
fn walk_dir(src_path: &Path, ext_name: &str, file: &mut File){
    // 获取目录下的所有文件或目录
    for entry in read_dir(src_path).ok().unwrap(){
        
        // entry.
        // 获取子文件或目录的路径对象
        let path = entry.unwrap().path();
        
        // 对文件进行处理
        if path.is_file() {
            // 获取文件的路径
            let ext = path.extension();
            if !ext.is_none() {
                
                let ext = ext.unwrap().to_str().unwrap();
                // println!("{:?}, {:?}", ext, ext_name);
                if ext.to_lowercase() == ext_name.to_lowercase() {
                    // let path = Path::new("ss.xx");
                    // path.to_str()
                    // // let path_str = path.display().to_str().unwrap();
                    // let path_str = format!("{0}", path.display()).as_str();
                    // println!("{}", path_str);
                    // println!("this is a Debug, {:?}", ext);
                    let path_str = format!("{}\n", path.to_str().unwrap());
                    // println!("ext unwrap end");
                    write_to_file(&path_str, file);
                    // info!("{}", path_str);
                }
            }
            // else{
            //     println!("{:?}", path.display());
            // }
        }
        // 目录进入再遍历
        else if path.is_dir() {
            
            let f_name = path.file_name().unwrap().to_str().unwrap();
            
            // println!("{:?}", f_name);
            if f_name == "node_modules" {
                // continue;
            }
            else if f_name.starts_with(".") {
                // continue
            }
            else{
                walk_dir(&path, ext_name, file);
            }
        }
    }
}

/// 写入文件 
fn write_to_file(src_path: &str, file: &mut File){
    match file.write_all(src_path.as_bytes()){
        Ok(_) => { 
            // println!("{:?}", s);
        },
        Err(_) => {}
    }
}