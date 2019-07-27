
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use time;
use mongodb::{bson, doc, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use chrono::{DateTime, FixedOffset};
// use chrono::prelude::*;Duration, 
use regex::Regex;
use std::path::{Path, PathBuf};
use std::thread;
// use std::io::prelude::*;

// macro_rules! try {
//     ($e:expr) => (match $e {
//         Ok(val) => val,
//         Err(err) => return Err(err),
//     });
// }

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

/// binlog日志分析结果
#[derive(Clone, Debug)]
struct AnalyzeResult {
    // sql执行时间 
    execute_time: i64,
    // 表名
    table_name: String,
    // sql语句
    command_text: String,
    // 执行类型
    execute_type: String
}

/// 分析结果
impl AnalyzeResult {

    /// 创建一个分析结果
    // fn new(execute_time: String, table_name: String, command_text: String, execute_type: String) -> Self {
    //     AnalyzeResult{
    //         execute_time,
    //         table_name,
    //         command_text,
    //         execute_type
    //     }
    // }

    /// 将定义的结构数据转换为Beson模式下的Document
    fn to_doc(&self) -> Document {
        let doc = doc!{
            "execute_time" => (self.execute_time),
            "execute_type"=> (&self.execute_type),
            "table_name"=> (&self.table_name),
            "command_text"=> (&self.command_text)
        };
        doc
    }
}

/// 解析日期时间
fn get_timestamp(d: &str, t: &str) -> i64 {
    let mut ex_t = String::from("20");
    let mut date_str = String::from(&d[0..2]);
    date_str.push_str("-");
    date_str.push_str(&d[2..4]);
    date_str.push_str("-");
    date_str.push_str(&d[4..]);
    ex_t.push_str(date_str.as_str());
    ex_t.push_str(" ");
    // let s = &d_str[0..2] + "-".to_string() + &d_str[2..4] + "-".to_string() + &d_str[4..] + " ".to_string();
    // 把时间处理为01这种模式
    let t_str = if t.len() == 7 {
        "0".to_string() + t
    }
    else{
        t.to_string()
    };

    ex_t.push_str(&t_str);
    ex_t.push_str(" +00:00");
    // execute_time = ex_t;
    let dt = DateTime::parse_from_str(&ex_t, "%Y-%m-%d %H:%M:%S %z");
    let china_timezone = FixedOffset::east(8 * 3600);
    // dt.unwrap().add(Duration::hours(8));
    // println!("{:?}, -===={:?}", dt, ex_t);
    let dt = dt.unwrap().with_timezone(&china_timezone);
    dt.timestamp_millis()
}

/// 读文件,按行读取
fn read_analyze_file(file_path: &str, db_conf: &DBConfig) {

    println!("开始对文件{:?}进行分析", file_path);
    let file = File::open(&file_path).expect("cannot open file");
    let reader = BufReader::new(file);
    let mut is_record = false;
    let mut count = 0;
    let mut single_sql: Vec<String> = vec![];
    let mut sqls: Vec<AnalyzeResult> = vec![];
    let mut table_name: String = String::from("tablename");
    let mut execute_type: String = String::from("insert");
    let mut execute_time: i64 = 0;

    // let 

    // reader.lines() 需要引用io::BufRead
    for line in reader.lines() {
        
        let line: String = line.unwrap();

        // 判断是否开始记录执行的Sql
        // 一个binlog的Sql开始于: BEGIN/*!*/; 结束于: COMMIT/*!*/;
        if is_record {

            // Sql语句每一行前面都是以### 开始
            if line.starts_with("### ") {
                single_sql.push(line.replace("### ", ""));
            }
            // 一条日志记录结束的判断
            else if line.starts_with("COMMIT"){
                is_record = false;
                // println!("{}", single_sql.join("\n"));

                let ar: AnalyzeResult = AnalyzeResult {
                    command_text: single_sql.join("\n"),
                    execute_time: execute_time,
                    execute_type: execute_type.clone(),
                    table_name: table_name.clone()
                };
                // 把单个sql语句加到sql集合中
                sqls.push(ar);
                // 重新给单个Sql赋值
                single_sql = vec![];

                // 每500 第进行一次入库处理
                if count % 100 == 0 {
                    println!("已分析{:?}条数据,即将进行入库.", count);
                    let x = sqls.clone();

                    write_to_mongo(&db_conf.clone(), x);
                    // 对象重新赋值
                    sqls = vec![];
                }
                println!("已处理行数:: {:?}", count);
                
            }
            // 判断表名类型
            else if !line.find("_map").is_none() {
                // split后需要使用collect()转换为Vector
                // #190713  2:38:15 server id 890708719  end_log_pos 422299 CRC32 0xe513b277 \tTable_map: `vast_saas_b2b_release`.`cor_codemaxno` mapped to number 52907
                let xx: Vec<&str> = line.split(" ").collect();
                // 日期部分
                let d_str = xx[0].replace("#", "");
                // 时间部分
                let t_str = if xx[1].len() == 0 {
                    xx[2]
                }
                else{
                    xx[1]
                };
                                
                // println!("line ::: {:?}", line);
                execute_time = get_timestamp(&d_str, &t_str);

                let re = Regex::new("[`].*[`]").unwrap();
                if re.is_match(&line) {
                    // println!("reg_map{:?}", re.captures(&line));
                    let caps = re.captures(&line).unwrap();
                    // println!("{}", caps.get(0).unwrap().as_str());
                    table_name = caps.get(0).unwrap().as_str().replace("`", "");
                }
                // dt.
                // let fixed_dt = dt.with_timezone(&FixedOffset::east(8*3600));
                // println!("xxxxxxx============{:?}=========={:?}===============", execute_time, dt);
            }
            // 判断操作类型
            else if !line.find("_rows").is_none(){
                let rtype = Regex::new("(Update|Delete|Write)_rows").unwrap();
                if rtype.is_match(&line) {
                    let caps = rtype.captures(&line).unwrap();
                    // println!("===================================={}=======================", caps.get(0).unwrap().as_str());
                    execute_type = caps.get(0).unwrap().as_str().replace("_rows", "");
                }
            }
            
        }
        else{
            if line.starts_with("BEGIN") {
                count = count + 1;
                is_record = true;
                
                // println!("{}", count);
            }
        }
        // println!("{}", line);
    }
    // 将最后的结果写入数据库中
    write_to_mongo(&db_conf.clone(), sqls);
    println!("文件 {:?} 分析结束, 总行数, {:?}", file_path, count);
}

/// 写入mongodb
fn write_to_mongo(db_conf: &DBConfig, data: Vec<AnalyzeResult>){

    println!("正在对{:?}条数据进行入库..", data.len());

    let client = Client::connect(&db_conf.server, db_conf.port)
        .expect("Failed to initialize standalone client.");

    let coll = client.db(&db_conf.database).collection("logs");

    let mut docs: Vec<Document> = vec![];
    for ar in data {
        docs.push(ar.to_doc());
    }
    // let doc = doc! {
    //     "title": "Jaws",
    //     "array": [ 1, 2, 3 ],
    // };

    // Insert document into 'test.movies' collection
    coll.insert_many(docs.clone(), None)
            .ok(); //.expect("Failed to insert document.");

    // client
    // std::panic::catch_unwind(|| {
    //         println!("{}", "Failed to insert document.");
    //     }
    // );
    // // Find the document and receive a cursor
    // let mut cursor = coll.find(Some(doc.clone()), None)
    //     .ok().expect("Failed to execute find.");

    // let item = cursor.next();

    // // cursor.next() returns an Option<Result<Document>>
    // match item {
    //     Some(Ok(doc)) => match doc.get("title") {
    //         Some(&Bson::String(ref title)) => println!("{}", title),
    //         _ => panic!("Expected title to be a string!"),
    //     },
    //     Some(Err(_)) => panic!("Failed to get next from server!"),
    //     None => panic!("Server returned no results!"),
    // }
}

/// 对目录中的文件进行分析
fn read_analyze_dir(dir_path: &str, db_conf: DBConfig, is_mutli_thread: bool) {
    println!("dir name: {:?}", dir_path);
    let dir = Path::new(dir_path);
    // // 提供一个 vector 来存放所创建的子线程（children）。
    let mut children = vec![];

    for entry in read_dir(dir).unwrap(){
        let p: PathBuf = entry.unwrap().path();
        if p.is_file() {
            let ext = p.extension();
            if !ext.is_none() {
                if ext.unwrap().to_str().unwrap().to_lowercase() == "sql" {
                    
                    if is_mutli_thread {
                        let conf = db_conf.clone();
                        // 启用多线程的方式进行文件分析
                        // 启动（spin up）另一个线程
                        children.push(thread::spawn(move || {
                            let f_path = p.to_str().unwrap();
                            read_analyze_file(&f_path, &conf);
                        }));
                    }
                    else{
                        let f_path = p.to_str().unwrap();
                        // println!("开始分析文件:{:?}.", f_path);
                        read_analyze_file(f_path, &db_conf);
                    }
                }
                else{
                    println!("文件 {:?} 不属于要分析的文件", p);
                }
            }
            
        }
    }

    if is_mutli_thread {
        // 等待线程结束
        for child in children {
            // 等待线程结束。返回一个结果。
            let _ = child.join();
        }
    }
    
}

fn main() {
    let c = Config::new("config.json");

    let start = time::now(); //获取开始时间
    
    // write_to_mongo(c.database);
    let f_path = c.root_dir; // "/users/shaipe/binlog";
    // let f_path = "/users/shaipe/react.sh";
    read_analyze_dir(&f_path, c.database, c.multi_thread);
    let end = time::now(); //获取结束时间
    println!(
        "done!start : {:?},end :{:?},duration:{:?}",
        start,
        end,
        end - start
    );
}
