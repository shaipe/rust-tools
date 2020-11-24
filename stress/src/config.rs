//! copyright © shaipe
//! Web配置功能模块
//!
// #[deny(dead_code)]

use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

// 服务器信息配置
#[derive(Debug, Clone, Deserialize)]
pub struct RequestConfig {
    // 名称
    pub name: String,
    // 请求方式
    pub method: String,
    // 请求地址
    pub url: String,
    // 发送数据
    pub data: String,
}

/// 获取配置文件
pub fn get_config(conf_path: &str, name: &str) -> Option<RequestConfig> {
    // 判断传入的conf_path是否为空,如果为空就给定默认值
    let file_path = if conf_path.len() == 0 {
        "config.conf"
    } else {
        conf_path
    };

    // 打开文件
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception: {}", file_path, e),
    };

    // 读取文件到字符串变量
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file:{}", e),
    };

    match serde_json::from_str(&str_val) {
        Ok(vc) => {
            let vrc: Vec<RequestConfig> = vc;
            for c in vrc {
                if name == "" {
                    return Some(c);
                } else if c.name == name.to_string() {
                    return Some(c);
                }
            }
            None
        }
        Err(_e) => None,
    }
}
