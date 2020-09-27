/**
 * 查找typescript项目中ts文件自动生成的map和js文件
 *
 */
use std::env;
use std::fs::{read_dir, remove_file};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg1 = if args.len() > 1 { &args[1] } else { "./" };

    let arg2 = if args.len() > 2 { &args[2] } else { "map" };

    let mut dir_path = "./";
    let mut target_name = arg2;
    if arg1.contains("/") {
        dir_path = arg1;
    } else {
        target_name = arg1;
    }

    walk_dir(Path::new(dir_path), target_name);
    println!("清理完成");
}

// 循环式删除目录下的所有targget目录
fn walk_dir(dir_path: &Path, ext_name: &str) {
    for entry in read_dir(dir_path).unwrap() {
        let p: PathBuf = entry.unwrap().path();

        if p.is_dir() {
            let dir_name = p.file_name().unwrap().to_str().unwrap();
            // 排除node_modules目录
            if dir_name == "node_modules" {
                continue;
            }
            walk_dir(&p, ext_name);
        } else {
            if let Some(t) = p.extension() {
                let ext = t.to_str().unwrap();
                // println!("{:?}", ext);
                let path_str = p
                    .as_path()
                    .to_str()
                    .unwrap()
                    .replace(&format!(".{}", ext), "");
                // println!("{:?}", path_str);
                if ext == ext_name {
                    if let Ok(_) = remove_file(&p) {
                        println!("{}", p.display());
                    };
                    if let Ok(_) = remove_file(Path::new(&path_str)) {
                        println!("{}", path_str);
                    };
                }
            };
        }
        //  if dir_name == target_name {
        //      match remove_dir_all(p.clone()){
        //          Ok(_) => {
        //              println!("删除目录: {:?} 成功!", p.display());
        //          }
        //          Err(e) => {
        //              println!("删除目录: {:?} 失败, 原因 {:?}!", p.display(), e);
        //          }
        //      }
        //  }
        //  else {
        //      if p.is_dir() {
        //          walk_dir(&p, target_name);
        //      }
        //  }
    }
}
