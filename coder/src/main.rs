/**
 * 代码生成器
 * by shaipe 20190727
 */
#[warn(unused_parens)]

// #[macro_use]
extern crate mysql;

// use mysql::from_row;
use std::env;

mod config;
mod field;
// mod xx;


/// 启动入口
fn main() {

    let args: Vec<String> = env::args().collect();

    // 获取启动输入的命令行参数值
    let conf_path = if args.len() > 1 {
        &args[1]
    }
    else{
        "config.json"
    };

    // 获取配置信息
    let c = config::Config::new(conf_path);
    println!("{:?}", c);

    let dbc = c.database;

    // 组建连接字符串
    let conn_str = format!("mysql://{}:{}@{}:{}/{}", dbc.user, dbc.password, dbc.server, dbc.port, dbc.db_name);

    let dbs = field::get_databases(&conn_str);
    println!("{:?}", dbs);

    let cls = field::get_columns(&conn_str, "ehr_category", &dbc.db_name);

    println!("{:?}", cls);

    let pool = mysql::Pool::new(conn_str).unwrap();

    for row in pool.prep_exec("Select * from ehr_category", ()).unwrap() {
        println!("{:?}", row);
    }
    
    // .map(| mut result| {
    //     let row = result.next().unwrap().unwrap();
    //     println!("{:?}", row);
    // });

    // pool.query("")

    // let conn = pool.get_conn().unwrap();

    // for row in pool.prep_exec("SELECT ?, ?", (42, 2.5)).unwrap() {
    //     let (a, b) = from_row(row.unwrap());
    //     println!("{:?} ", (a, b));
    //     // assert_eq!((a, b), (42u8, 2.5_f32));
    // }

    // pool.prep_exec("SELECT * from ehr_category", ())
    // .map(|result|{
    //     result.map(|x| x.unwrap())
    //     .map(|row| {
    //         let (categoryname) = my::from_row(row);
    //         println!("{:?}", categoryname);
    //         categoryname
    //     }).collect()
    // }).unwrap();
    // println!("{:?}", xx);
    
    // Let's select payments from database
    // let selected_payments: Vec<String> =
    // pool.prep_exec("SELECT * from ehr_category", ())
    // .map(|result| { 
    //     // In this closure we will map `QueryResult` to `Vec<Payment>`
    //     // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
    //     // will map each `MyResult` to contained `row` (no proper error handling)
    //     // and second call to `map` will map each `row` to `Payment`
    //     println!("{:?}", result);
    //     result.map(|x| x.unwrap()).map(|row| {
    //         let (categoryname) = my::from_row(row);
    //          categoryname
    //     }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    // }).unwrap(); // Unwrap `Vec<Payment>`

    // println!("{:?}", selected_payments);

    // // Let's create payment table.
    // // It is temporary so we do not need `tmp` database to exist.
    // // Unwap just to make sure no error happened.
    // pool.prep_exec(r"CREATE TEMPORARY TABLE tmp.payment (
    //                      customer_id int not null,
    //                      amount int not null,
    //                      account_name text
    //                  )", ()).unwrap();


    println!("Hello, world!");
}
