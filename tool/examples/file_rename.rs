/**
 * 文件重命名
 * by shaipe 20190729
 * 参数说明,1: 待替换文件类型, 2: 替换为xx类型 3: 待处理目录
 * ./file_rename mp4 mp ./
 */

use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
// use std::io::prelude::*;
use std::fs::rename;
use regex::Regex;

/// 入口函数
fn main() {
    let args: Vec<String> = env::args().collect();
    

    // 待修改的文件类型
    let src_ext = if args.len() > 1 {
        &args[1]
    }
    else{
        "txt"
    };

    // 给定修修改为的目录文件类型
    let dist_ext = if args.len() > 2 {
        &args[2]
    }
    else{
        "log"
    };

    // 源目录
    let src_dir = if args.len() > 3 {
        &args[3]
    }
    else{
        "./"
    };

    // let dist_dir = if args.len() > 4 {
    //     &args[4]
    // }
    // else{
    //     ""
    // };
    // println!("{:?}", src_dir);

    walk_dir(Path::new(src_dir), src_ext, dist_ext);
    // println!("{:?}, {:?}, {:?}", dist_dir, src_ext, dist_ext);
}

/// 通过递归的方式遍历所有的目录
fn walk_dir(src_path: &Path, parten: &str, dist_txt: &str) {
    let reg_parten = parten.replace("[", "\\[").replace("]", "\\]").replace(".", "\\.");
    // println!("{}", reg_parten);
    let reg: Regex = Regex::new(&reg_parten).unwrap();

    // 获取当前目录下的所有文件或目录
    for entry in read_dir(src_path).unwrap(){
        let path: PathBuf = entry.unwrap().path();
        // println!("{:?}", path.to_str());
        // let path_str = path.display();
        // 针对文件进行处理
        if path.is_file() {
            let f_name: &str = path.file_name().unwrap().to_str().unwrap();
            let t_name: String = format!("{}", reg.replace(f_name.clone(), dist_txt)); // .as_str();
            // println!("{},{},{}", f_name, dist_txt, t_name);
            let dir_path = path.parent().unwrap();
            let file_path = dir_path.clone().join(f_name);
            let t_file_path = dir_path.join(t_name);
            
            let file_path = file_path.to_str().unwrap();
            let t_file_path = t_file_path.to_str().unwrap();
            // println!("{:?}, {:?}", file_path, t_file_path);
            match rename(file_path, t_file_path) {
                Ok(_) => println!("rename {:?} to {:?} successfully!",file_path , t_file_path),
                Err(err) => println!("{:?}", err)
            };


            // let ext = path.extension();
            // // 判断文件是否有扩展名,只对有扩展名的文件进行分类
            // if !ext.is_none() {
            //     let ext_str = ext.unwrap().to_str().unwrap();

            //     if ext_str.to_lowercase() == src_ext {
            //         let file_name = path.file_name().unwrap().to_str().unwrap();
            //         let t_file_name = file_name.clone();
            //         // let t_file_name = file_name.clone().replace(src_ext, dist_ext);
            //         let dir_path = path.parent().unwrap();

            //         let file_path = dir_path.join(file_name).to_str().unwrap();
            //         let t_file_path = dir_path.join(t_file_name).to_str().unwrap();

            //         println!("{:?}, {:?}, {:?}", dir_path.join(file_name), file_path, t_file_path);
            //         match rename(file_path, t_file_path) {
            //             Ok(_) => println!("rename {:?} to {:?} successfully!",file_path , t_file_path),
            //             Err(err) => println!("{:?}", err)
            //         };
            //     }
            // }
            
        }
        else if path.is_dir() {
            // 递归处理子目录
            walk_dir(&path, parten, dist_txt);
        }
    }
}