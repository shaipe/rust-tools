// Copyright © Shaipe

//! The MIT License (MIT)

use serde_derive::Deserialize;
use std::io::prelude::*;
use std::fs::File;


use lane_mysql::{MysqlConfig, set_db_config};

// 业务配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub title: String,
    pub upload_dir: String,
    pub mysql: Option<MysqlConfig>
}


impl Config {
    // 加载配置
    pub fn new(config_path: &str) -> Self {
        let file_path =  if config_path.is_empty() {
            "config.conf"
        }
        else{
            config_path
        };

        // 打开文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e)
        };

        // 读取文件到字符串变量
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e)
        };
       
        let c: Config = serde_json::from_str(&str_val).unwrap();
        c
    }
}

// 获取配置信息
pub fn get_config(mode: &str)->MysqlConfig {
    let config_path = match mode {
        "dev" => "dev.conf",
        _ => "prod.conf"
    };
    let cnf: Config = Config::new(config_path);
    // 将数据库的配置写入数据库底层配置中
   let sql_config= match cnf.clone().mysql {
        Some(val) => {
            set_db_config(val.clone());
            val
        },
        _ => {
            println!("数据库连接没有配置哟!");
            MysqlConfig::default()
        }
    };

    sql_config
}

