#![allow(non_snake_case)]
#![deny(warnings)]

use std::{fs};
// use std::env;
use std::path::{Path};
// use clap::{Arg, App}; // value_t
use std::time::SystemTime;
use chrono::{DateTime, offset::Local as LocalTz}; //SubsecRound,

mod config;

fn main() {

    // 配置需要输入的参数管理器
    // let matches = App::new("Deployer")
    //     .version("0.1.0")
    //     .arg(
    //         // 输入config路径
    //         Arg::with_name("config")
    //             .short("c")
    //             .long("config")
    //             .value_name("FILE")
    //             .help("Sets a custom config file")
    //             .takes_value(true)
    //     )
    //     .arg(
    //         Arg::with_name("start_time")
    //             .takes_value(true)
    //             .value_name("输入开始时间")
    //             // .index(2)
    //             .required(false)
    //     )   
    //     .get_matches();
    
    // let listen_addr = matches.value_of("config").unwrap();

    // println!("{:?}", listen_addr);
    // let listen_port = value_t!(matches, "listen_port", u16).unwrap_or_else(|e| e.exit());

    let c = config::Config::new("config.json");
    println!("{:?}", c);
    let p = Path::new("./");
    println!("{:?}", p.canonicalize().unwrap().as_path());
    // println!("curdir: {}, curexe: {}", env::current_dir().unwrap().display(), env::current_exe().unwrap().display());

    // println!("cd to ../..\n");
    // // 设置当前工作目录
    // env::set_current_dir("/users/shaipe/dist/").unwrap();
    // println!("ls: {}", env::current_dir().unwrap().display());

    // let paths = fs::read_dir("./").unwrap();
    // for path in paths {
    //     let f = path.unwrap().path();
         

    //     println!("{} {}", if f.is_file() { "f" } else { "d" }, f.display());
    // }
    // let ss = str::

    let x = String::from("s: &String");
    let x = x.replace("s", "tos");
    let start = SystemTime::now();

  

    println!("x:: {} , {:?}", x, start);

    let p = Path::new("/users/shaipe/dist/");
    walk_dir(&p);


}


fn walk_dir(dir: &Path) {
    for entry in fs::read_dir(dir).unwrap() {
        let ent = entry.unwrap();
        let path = ent.path();
        let data = ent.metadata().unwrap();
        if path.is_dir(){
            walk_dir(&path);
        }
        else{

            if let Ok(modified) = data.modified() {
                let modified: DateTime<LocalTz> = modified.into();
                println!("{:?}", modified);
            }
            if let Ok(time) = data.modified() {
                println!("{:?}", time);
            }
            // if data.is_file() {
            //     if let Some(ex) = path.extension() {
            //         if ex == "js" && data.len() > 1024 {
            //             println!("{} length {}", path.display(),data.len());
            //         }
            //     }
            // }
            // println!("{:?}", data);
        }
    }
}