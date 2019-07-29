/**
 * 文件重命名
 * by shaipe 20190729
 */

use std::env;
use std::fs::read_dir;
use std::path::Path;
// use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src_dir = if args.len() > 1 {
        &args[1]
    }
    else{
        "./"
    };

    let src_ext = if args.len() > 2 {
        &args[2]
    }
    else{
        "txt"
    };

    let dist_ext = if args.len() > 3 {
        &args[3]
    }
    else{
        "log"
    };

    let dist_dir = if args.len() > 4 {
        &args[4]
    }
    else{
        ""
    };
    // println!("{:?}", src_dir);

    walk_dir(Path::new(src_dir), src_ext);
    println!("{:?}, {:?}, {:?}", dist_dir, src_ext, dist_ext);
}

fn walk_dir(src_path: &Path, src_ext: &str) {

    for entry in read_dir(src_path).unwrap(){
        let path = entry.unwrap().path();
        let path_str = path.display();

        if path.is_file() {
            let ext = path.extension();
            // 判断文件是否有扩展名,只对有扩展名的文件进行分类
            if !ext.is_none() {
                let ext_str = ext.unwrap().to_str().unwrap();
                if ext_str.to_lowercase() == src_ext {
                    println!("{:?}", path_str);
                }
            }
            
        }
        else if path.is_dir() {
            walk_dir(&path, src_ext);
        }
    }
}