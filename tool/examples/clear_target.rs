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

    let dir_path = if args.len() > 1 {
        &args[1]
    }
    else {
        "./"
    };

    walk_dir(Path::new(dir_path));
    println!("清理完成");
}

// 循环式删除目录下的所有targget目录
fn walk_dir(dir_path: &Path) {

    for entry in read_dir(dir_path).unwrap() {
        let p: PathBuf = entry.unwrap().path();

        let dir_name = p.file_name().unwrap().to_str().unwrap();
        if dir_name == "target" {
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
                walk_dir(&p);
            }
        }
    }
}