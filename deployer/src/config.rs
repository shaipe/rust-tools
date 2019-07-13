use std::fs::File;
use std::io::prelude::*;
use serde_derive::Deserialize;

/// 部署工具配置
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // 源目录
    pub src: String,
    // 目标目录
    pub dist: String,
    // 是否先清除目标目录
    pub clear: bool,
    // 需要排除的目录
    pub exclude_dir: Vec<String>,
    // 要排队的扩展类型
    pub exclude_ext: Vec<String>,
    // 要排除的文件名
    pub exclude_file: Vec<String>,
}

/// 创建并使用配置文件
impl Config {

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