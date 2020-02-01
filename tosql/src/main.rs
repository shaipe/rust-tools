//! copyrigt (c) 2020 by shaipe
//! 从rs实体文件中提取字段创建数据库表结构
//!

use lane::fs::{append_content, read_content};
use std::env;
use std::path::{Path, PathBuf};

mod mysql;
mod postgres;

/// 入口函数
fn main() {
    // 获取输入参数
    let args: Vec<String> = env::args().collect();

    let prefix = if args.len() > 1 { &args[1] } else { "" };

    let dir_str = if args.len() > 2 { &args[2] } else { "./" };

    let sql_type = if args.len() > 3 { &args[3] } else { "mysql"};

    // 获取目录路径
    let dir_path = Path::new(dir_str);
    let file_path = dir_path.join("./tables.sql");
    let file_path_str = format!("{}", file_path.display());

    println!("开始对 {:?} 目录进行处理, 将把结果写入 {:?} 文件中.", dir_path.display(), file_path.display());

    walk(PathBuf::from(dir_path), &file_path_str, sql_type, prefix);

    println!("代码生成已经完成");
}

/// 数据库接口
pub trait Sql {

    /// 获取数据类型
    fn get_data_type(&self, type_str: &str) -> String;

    /// 获取自增长
    fn get_identifier(&self) -> String;

    /// 获取数据库语句
    fn get_sql(&self, content: String, prefix: &str) -> String;
}

/// 遍历整个目录对指定的rs文件进行处理
fn walk(dir_path: PathBuf, file_path: &str, sql_type: &str, prefix: &str) {
    if dir_path.exists() {
        for entry in dir_path.read_dir().expect("指定的路径 {} 不存在.") {
            let p = entry.unwrap().path();
            // println!("{:?}", p);
            if p.is_file() {
                let ext = p.extension();
                // 判断只处理后缀为rs的文件
                if !ext.is_none() && ext.unwrap() == "rs" {
                    // 获取路径的字符
                    let p_str = format!("{}", p.display());
                    // 判断只处理src目录下的文件
                    if p_str.contains(r"/src/") {
                        let content = read_content(&p_str);
                        let sql = if sql_type == "postgres" {
                            postgres::Postgres{}.get_sql(content, prefix)
                        }
                        else{
                            mysql::MySql{}.get_sql(content, prefix)
                        };
                        // let sql = get_create_sql(content, prefix);
                        append_content(file_path, sql.as_bytes());
                    }
                }
            } else {
                walk(p, file_path, sql_type, prefix);
            }
        }
    } else {
        println!("指定的路径 {} 不存在.", dir_path.display());
    }
}
