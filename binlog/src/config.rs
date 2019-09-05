use std::fs::File;
use std::io::prelude::*;
use serde_derive::Deserialize;

/// binlog 日志分析工具配置信息
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    // 数据库配置
    pub database: DBConfig,

    // 分析日志的根目录
    pub root_dir: String,

    // 是否启用多线程
    pub multi_thread: bool
}

#[derive(Debug, Deserialize, Clone)]
pub enum DatabaseType {
    Mongo,
    Mysql
}


/// mongo数据库配置
#[derive(Debug, Deserialize, Clone)]
pub struct DBConfig {
    // 服务器地址
    pub server: String,
    // mongo数据库端口
    pub port: u16,
    // 数据库名称
    pub database: String,

    pub db_type: DatabaseType
}

/// 基础配置类功能实现
impl Config {

    /// 读取配置信息
    pub fn new(conf_path: &str) -> Self {
        // 给定默认值
        let file_path = if conf_path.len() == 0 {
            "config.json"
        }
        else {
            conf_path
        };

        // 打开配置文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e)
        };

        // 读取文件到字符串变量, read_to_string 需要使用: use std::io::prelude::*;
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e)
        };

        // serde_json 需要引用
        let jc: Config = serde_json::from_str(&str_val).unwrap();

        jc
    }
}
