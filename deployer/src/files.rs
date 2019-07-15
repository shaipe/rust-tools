// （一）从文件按行读取内容，打印输出

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn readline_and_print() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        if let Ok(line) = line {
            println!("{:?}", line);
        }
    }
    Ok(())
}

fn main() {
    match readline_and_print() {
        Ok(()) => println!("it's ok"),
        Err(err) => println!("zzz: {}", err)
    }
}
// （二）将内容写入文件

// 打开文件可以指定多个参数，以下例子可读可写，create(true)是如果文件不存在则创建文件，存在则使用这个文件，create_new(true)的作用是，当文件存在时，会报错，Error { repr: Os { code: 17, message: "File exists" } }，不存在则新建文件，并且指定append追加写入，打开文件，将文件句柄赋值给file.

use std::io;
use std::io::prelude::*;
use std::fs::OpenOptions;

fn run() -> io::Result<()> {
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

fn main() {
    match run() {
        Ok(()) => println!("It's ok"),
        Err(err) => println!("zzz: {}", err)
    }
}
// （三）获取目录列表

// 对文件进行操作，很可能会读取目录列表，使用fs::read_dir方法，可以获取目录列表及文件相关属性

use std::fs;

fn run() {
    if let Ok(entries) = fs::read_dir("photos") {
        for entry in entries {
            println!("{:?}", entry);
            if let Ok(entry) = entry {
                println!("{:?}", entry.path());
                println!("{:?}", entry.file_name());
                println!("{:?}", entry.file_type());
            }
        }
    }
}

fn main() {
    run();
}
// （四）复制文件

// 在本例中，复制指定文件，并放在指定文件夹(funny)下，文件命名为“funny_1514532002.db”，后边的数字为时间戳

use std::fs;
use std::io;
use std::path::Path;
extern crate time;

// copy file to other area

fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

fn aha() -> io::Result<()> {

    let ts = timestamp();
    let ts_str = ts.to_string();

    let v: Vec<&str> = ts_str.split(".").collect();
    let filename = "funny/funny_".to_owned() + v[0] + ".db";
    println!("file name: {}", filename);

    if !Path::new("funny").exists() {
        fs::create_dir("funny")?;
    }

    fs::copy("/root/test.db", filename)?;
    Ok(())
}

fn main() {
    match aha() {
        Ok(s) => println!("ok: {:?}", s),
        Err(err) => println!("err: {:?}", err),
    }
}