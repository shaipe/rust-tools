/**
 * 清理rust项目目录下所有的target目录
 * 由于rust开发过程中产生的target目录相对比较大,对磁盘空间的占用较多,故写这么一个工具来清理
 * 
 */

use std::env;
use std::fs::{read_dir, remove_dir_all};
use std::path::{Path, PathBuf};


fn main() {
    let args: Vec<String> = env::args().collect();

    let arg1 = if args.len() > 1 {
        &args[1]
    }
    else {
        "./"
    };

    let arg2 = if args.len() > 2 {
        &args[2]
    }
    else {
        "target"
    };

    let mut dir_path = "./";
    let mut target_name = arg2;
    if arg1.contains("/") {
        dir_path = arg1;
    }
    else {
        target_name = arg1;
    }

    walk_dir(Path::new(dir_path), target_name);
    println!("清理完成");
}

// 循环式删除目录下的所有targget目录
fn walk_dir(dir_path: &Path, target_name: &str) {

    for entry in read_dir(dir_path).unwrap() {
        let p: PathBuf = entry.unwrap().path();

        let dir_name = p.file_name().unwrap().to_str().unwrap();
        if dir_name == target_name {
            match remove_dir_all(p.clone()){
                Ok(_) => {
                    println!("删除目录: {:?} 成功!", p.display());
                }
                Err(e) => {
                    println!("删除目录: {:?} 失败, 原因 {:?}!", p.display(), e);
                }
            }
        }
        else {
            if p.is_dir() {
                walk_dir(&p, target_name);
            }
        }
    }
}