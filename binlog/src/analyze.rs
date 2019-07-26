use regex::Regex;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::thread;
use chrono::{DateTime, FixedOffset};

use crate::analyze_result::AnalyzeResult;
use crate::dbase::DBase;

pub struct Analyze {
    pub dir: String,
    pub db: DBase
}

impl Analyze {

    pub fn new(dir_str: &str, db: DBase) -> Self {
        Analyze {
            dir: dir_str.to_string(),
            db: db
        }
    }

    /// 读文件,按行读取
    pub fn analyze_file(&self, file_path: &str) {

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
                    if count % 500 == 0 {
                        println!("已分析{:?}条数据,即将进行入库.", count);
                        let x = sqls.clone();
                        println!("{:?}", x);
                        // write_to_mongo(&db_conf.clone(), x);
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
                    execute_time = self.get_timestamp(&d_str, &t_str);

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
        // write_to_mongo(&db_conf.clone(), sqls);
        self.db.insert(sqls);
        println!("文件 {:?} 分析结束, 总行数, {:?}", file_path, count);
    }

    /// 对目录中的文件进行分析
    pub fn read_analyze_dir(&self, is_mutli_thread: bool) {
        println!("dir name: {:?}", self.dir);
        let dir = Path::new(self.dir.as_str());
        // // 提供一个 vector 来存放所创建的子线程（children）。
        let mut children = vec![];

        for entry in read_dir(dir).unwrap(){
            let p: PathBuf = entry.unwrap().path();
            if p.is_file() {
                let ext = p.extension();
                if !ext.is_none() {
                    if ext.unwrap().to_str().unwrap().to_lowercase() == "sql" {
                        
                        if is_mutli_thread {
                            // let conf = db_conf.clone();
                            // // 启用多线程的方式进行文件分析
                            // // 启动（spin up）另一个线程
                            children.push(thread::spawn(move || {
                                // let f_path = p.to_str().unwrap();
                                // self.analyze_file(&f_path);
                            }));
                        }
                        else{
                            let f_path = p.to_str().unwrap();
                            // // println!("开始分析文件:{:?}.", f_path);
                            self.analyze_file(f_path);
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
            // unsafe {
            //     COMPLETED = true;
            // }
        }
        
    }

    /// 解析日期时间
    fn get_timestamp(&self, d: &str, t: &str) -> i64 {
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

}