/**
 * 对目录下的文件进行自动分类到类型目录
 * by shaipe 20190726
 */

use std::env;
use std::path::Path;
use std::fs::{read_dir, create_dir};


fn main(){
    let args: Vec<String>= env::args().collect();

    let dir_str = if args.len() > 1 {
        &args[1]
    }
    else {
        "./"
    };

    // 获取当前目录下的所有文件或目录
    for entry in read_dir(dir_str).unwrap(){
        let path = entry.unwrap().path();
        // 只对当前目录下的文件进行分类处理
        if path.is_file() {
            let ext = path.extension();
            // 判断文件是否有扩展名,只对有扩展名的文件进行分类
            if !ext.is_none() {
                let ext_str = ext.unwrap().to_str().unwrap();
                // 获取扩展类型对应的目录串
                let t_dir_str = dir_str.to_owned() + ext_str;
                let t_path = Path::new(&t_dir_str);
                // 判断目录是否存在,如果不存在就创建目录
                if !t_path.exists() {
                    // create_dir(ext_str);
                }
                let tf_str = t_dir_str + "/" + path.file_name().unwrap().to_str().unwrap();
                println!("{:?}", tf_str);
            }
            else{
                println!("file {:?} not extension!", path.display());
            }
        }
    }

}