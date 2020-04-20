// Copyright © Shaipe

//! The MIT License (MIT)

use serde_derive::Deserialize;
use std::io::prelude::*;
use std::fs::File;


use lane_mysql::{MysqlConfig, set_db_config};
#[derive(Debug, Clone, Deserialize)]
pub struct WebConfig{
    pub api_domain:String,
    pub sn:String,
    pub db_id:u16,
    pub app_id:i32,
    pub version_apps:Vec<i32>,
    pub app_name:String
}
impl WebConfig{
    pub fn default()->Self{
        WebConfig{
            api_domain:String::from(""),
            sn:String::from(""),
            db_id:0,
            app_id:0,
            version_apps:Vec::new(),
            app_name:String::from("")
        }
    }
}
// 业务配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub title: String,
    pub upload_dir: String,
    pub web:Option<WebConfig>,
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
pub fn get_config(mode: &str)->WebConfig {
    let config_path = match mode {
        "dev" => "dev.conf",
        _ => "prod.conf"
    };
    let cnf: Config = Config::new(config_path);
    // 将数据库的配置写入数据库底层配置中
   match cnf.clone().mysql {
        Some(val) => {
            set_db_config(val.clone());
        },
        _ => {
            println!("数据库连接没有配置哟!");
        }
    };

    let web_config=match cnf.clone().web{
        Some(val) => {
            val
        },
        _ => {
            println!("数据库连接没有配置哟!");
            WebConfig::default()
        }
    };
    web_config
}

