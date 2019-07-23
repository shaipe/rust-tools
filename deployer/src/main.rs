#![allow(non_snake_case)]
#![deny(warnings)]

use std::{fs};
// use std::env;
use std::path::{Path};
// use clap::{Arg, App}; // value_t
// use std::time::SystemTime;
// use chrono::{DateTime, offset::Local as LocalTz}; //SubsecRound,
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
// use std::path::Path;

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

    // println!("{:?}", c);
    // let p = Path::new("./");
    // println!("{:?}", p.canonicalize().unwrap().as_path());

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

    // let x = String::from("s: &String");
    // let x = x.replace("s", "tos");
    // let start = SystemTime::now();

  

    // println!("x:: {} , {:?}", x, start);

    let p = Path::new(&c.src);
    println!("{:?}", p);

    let write_path = format!("{}/files.csv", &c.dist);
    println!("{}", write_path);
    let mut file = match File::create(&write_path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    walk_dir(&p, &mut file);


}

/// 目录遍历
fn walk_dir(dir: &Path, file: &mut File) {
    for entry in fs::read_dir(dir).unwrap() {
        let ent = entry.unwrap();
        let path = ent.path();
        // let data = ent.metadata().unwrap();
        if path.is_dir(){
            walk_dir(&path, file);
        }
        else{

            let path_str = path.display();
            let display = format!("{:?}\n", path_str);
            // println!("{:?}", format!("{:?}", path_str).find("_"));
            if display.find("_").is_none() {
                // println!("{}", path.display());
                match file.write_all(display.as_bytes()) {
                    Err(why) => {
                        panic!("couldn't write to : {}", why.description())
                    },
                    Ok(_) => {println!("successfully wrote to {}", display)},
                };
            }
            
            // // Set `Last-Modified` and check `If-Modified-Since`.
            // if let Ok(modified) = metadata.modified() {
            //     let modified: DateTime<LocalTz> = modified.into();

            //     match self.if_modified_since {
            //         // Truncate before comparison, because the `Last-Modified` we serve
            //         // is also truncated through `DateTime::to_rfc2822`.
            //         Some(v) if modified.trunc_subsecs(0) <= v.trunc_subsecs(0) => {
            //             return ResponseBuilder::new()
            //                 .status(StatusCode::NOT_MODIFIED)
            //                 .body(Body::empty())
            //         },
            //         _ => {},
            //     }

            //     res.header(header::LAST_MODIFIED, modified.to_rfc2822().as_str());
            //     res.header(header::ETAG, format!("W/\"{0:x}-{1:x}.{2:x}\"",
            //         metadata.len(), modified.timestamp(), modified.timestamp_subsec_nanos()).as_str());
            // }
            
            
            // begin
            // let dt = DateTime::parse_from_str("2019-07-01 10:31:19 +08:00", "%Y-%m-%d %H:%M:%S %z");

            // if let Ok(modified) = data.modified() {
            //     let modified: DateTime<LocalTz> = modified.into();
            //     // Some(dt) if modified.trunc_subsecs(0) >= dt.trunc_subsecs(0) => {
            //     //     return "x"
            //     // }

            //     println!("{:?}, dt : {:?}", modified, dt);
            // }
            // if let Ok(time) = data.modified() {
            //     println!("{:?}", time);
            // }
            // end



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