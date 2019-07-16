
use std::fs::File;
use std::io::{BufRead, BufReader};
use time;
use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

mod config;
use config::{Config, DBConfig};

// /// binlog日志分析结果
// struct AnalyzeResult {
//     // sql执行时间 
//     execute_time: String,
//     // 表名
//     table_name: String,
//     // sql语句
//     command_text: String,
//     // 执行类型
//     excute_type: String
// }

fn read_all_lines(file_path: &str) {

    let file = File::open(&file_path).expect("cannot open file");

    let reader = BufReader::new(file);

    let mut is_record = false;

    let mut count = 0;

    let mut single_sql: Vec<String> = vec![];

    let mut sqls: Vec<String> = vec![];

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
            else if line.starts_with("COMMIT"){
                is_record = false;
                println!("{}", single_sql.join("\n"));
                // 把单个sql语句加到sql集合中
                sqls.push(single_sql.join("\n"));
                // 重新给单个Sql赋值
                single_sql = vec![];
                
            }
            // 判断表名类型
            else if !line.find("_map").is_none() {
                // split后需要使用collect()转换为Vector
                // #190713  2:38:15 server id 890708719  end_log_pos 422299 CRC32 0xe513b277 \tTable_map: `vast_saas_b2b_release`.`cor_codemaxno` mapped to number 52907
                let xx: Vec<&str> = line.split(" ").collect();
                println!("xxxxxxx======================{:?}, {:?}", xx, line);
            }
            // 判断操作类型
            else if !line.find("_rows").is_none(){

            }
            // else{

            // }
            
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
    
    println!("总行数, {:?}", sqls.len());
}

fn write_to_mongo(db_conf: DBConfig){

    println!("{:?}", db_conf);

    let client = Client::connect(&db_conf.server, db_conf.port)
        .expect("Failed to initialize standalone client.");

    let coll = client.db(&db_conf.database).collection("movies");

    let doc = doc! {
        "title": "Jaws",
        "array": [ 1, 2, 3 ],
    };

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");

    // Find the document and receive a cursor
    let mut cursor = coll.find(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();

    // cursor.next() returns an Option<Result<Document>>
    match item {
        Some(Ok(doc)) => match doc.get("title") {
            Some(&Bson::String(ref title)) => println!("{}", title),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
}

fn main() {
    let c = Config::new("config.json");

    println!("{:?}", c);

    let start = time::now(); //获取开始时间
    println!("开始进行日志分析, {:?}", "sd");
    write_to_mongo(c.database);
    // let f_path = "/users/shaipe/binlog/mysql-bin.003908.sql";
    let f_path = "/users/shaipe/react.sh";
    read_all_lines(&f_path);
    let end = time::now(); //获取结束时间
    println!(
        "done!start : {:?},end :{:?},duration:{:?}",
        start,
        end,
        end - start
    );
}
