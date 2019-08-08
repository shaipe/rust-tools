use std::env;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::io::prelude::*;

fn main() {
    let args : Vec<String> = env::args().collect();

    // 源目录
    let file_path = if args.len() > 1 {
        &args[1]
    }
    else{
        panic!("please input file path!");
    };

    let dist_path = if args.len() > 2 {
        &args[2]
    }
    else{
        "result.log"
    };

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => panic!("error {:?}", e)
    };

    let mut dist_file = match File::create(dist_path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let re = Regex::new("^.*(100\\.116\\.\\d{1,3}\\.\\d{1,3}).*$").unwrap();

    for line in reader.lines() {
        let line: String = line.unwrap();
        if !re.is_match(&line) {
            match dist_file.write_all(format!("{}\n", line).as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to : {}", why)
                },
                Ok(_) => {println!("successfully wrote to {}", line)},
            };
        }
        else{
            println!("{}", line);
        }
       
    }

    println!("{}", file_path);
}
