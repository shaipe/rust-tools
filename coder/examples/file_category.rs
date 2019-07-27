/**
 * 对目录下的文件进行自动分类到类型目录
 * by shaipe 20190726
 */

use std::env;
use std::path::Path;
use std::fs::{read_dir, create_dir, copy, remove_file};

use std::io;
use std::io::prelude::*;
use std::fs::OpenOptions;


fn main(){
    // 获取输入的参,参数为待处理的路径
    let args: Vec<String>= env::args().collect();

    let dir_str = if args.len() > 1 {
        &args[1]
    }
    else {
        "./"
    };
    // test_write_file();
    // 获取当前目录下的所有文件或目录
    for entry in read_dir(dir_str).unwrap(){
        let path = entry.unwrap().path();
        // 只对当前目录下的文件进行分类处理
        if path.is_file() {
            let ext = path.extension();
            // 判断文件是否有扩展名,只对有扩展名的文件进行分类
            if !ext.is_none() {
                let ext_str = ext.unwrap().to_str().unwrap();
                // println!("{}", ext_str);
                // 测试时使用,只移动txt文件
                // if ext_str.to_lowercase() != "txt"{
                //     continue;
                // }
                // 获取扩展类型对应的目录串
                let t_dir_str = dir_str.to_owned() + ext_str;
                let t_path = Path::new(&t_dir_str);
                // println!("{:?}", t_dir_str);

                // 判断目录是否存在,如果不存在就创建目录
                if !t_path.exists() {
                    let _ = create_dir(t_dir_str.clone());
                }
                let tf_str = t_dir_str + "/" + path.file_name().unwrap().to_str().unwrap();
                
                // 复制文件
                match copy(path.clone(), tf_str.clone()){
                    Ok(_)=>{
                        // 复制文件成功后删除源文件
                        match remove_file(path.clone()){
                            Ok(_)=>{},
                            Err(ex) => println!("{:?}", ex)
                        }
                    },
                    Err(ex) => println!("{:?}", ex)
                }
                
                println!("move file {:?}, to {:?} successfully!", path.display(), tf_str.clone());
            }
            else{
                println!("file {:?} not extension!", path.display());
            }
        }
    }

}

fn test_write_file() -> io::Result<()> {
    let filename = "foo.txt";
    let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                //.create_new(true)
                .append(true)
                .open(filename);

    match file {
        Ok(mut stream) => {
            stream.write_all(b"hello, world!\n")?;
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    Ok(())
}