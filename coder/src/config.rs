use std::fs::File;
use serde_derive::Deserialize;
use std::io::prelude::*;

/// 配置对象
#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    // 数据库配置
    pub database: Database
}

/// 数据库配置信息
#[derive(Debug, Deserialize)]
pub struct Database {
    // 数据库类型
    pub db_type: String,
    // 服务器地址
    pub server: String,
    // 端口
    pub port: i32,
    // 用户名
    pub user: String,
    // 密码
    pub password: String,
    // 数据库名称
    pub db_name: String
}


impl Config {

    // 创建Config并加载相应信息
    pub fn new(conf_path: &str) -> Self {
        // println!("{}", conf_path);

        // 判断传入的conf_path是否为空,如果为空就给定默认值 
        let file_path = if conf_path.len() == 0 {
            "config.toml"
        } else {
            conf_path
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
       
        // println!("file path {:?}", file_path);

        // if file_path.ends_with(".json") {

        // }
        // else {
        //     // 使用toml载加配置信息到结构体中
        //     let tc: Config = toml::from_str(&str_val).unwrap();
        //         Self {
        //         title: tc.title,
        //         server: tc.server,
        //         proxy: tc.proxy
        //     }
        // }
        let jc: Config = serde_json::from_str(&str_val).unwrap();
        Self {
            title: jc.title,
            database: jc.database
        }
    }
}