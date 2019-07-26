#![allow(non_snake_case)]
#![deny(warnings)]
#[deny(unused_imports)]

use time;

mod config;
mod queue;
mod convert;
mod analyze;
mod analyze_result;
mod dbase;

pub use analyze_result::AnalyzeResult;
use analyze::read_analyze_dir;
pub use config::Config;
pub use queue::Queue;
pub use convert::Convert;
use dbase::DBase;


fn main() {
    
    let start = time::now(); //获取开始时间
    let c = Config::new("config.json");
    let cc = c.clone();

    let db = DBase::new(cc.database);
    read_analyze_dir(&c.root_dir, db, c.multi_thread);

    // unsafe{
    //     SQL_QUEUE = Some(Queue::new());
    // }

    // // 启动一个线程来处理待插入数据库的队列
    // thread::spawn(move || {
    //     write_to_mongo(&cc.clone().database, vec![]);
    // });
    

    // write_to_mongo(c.database);
    // let f_path = c.root_dir; // "/users/shaipe/binlog";
    // let f_path = "/users/shaipe/react.sh";
    // read_analyze_dir(&f_path, c.database, c.multi_thread);
    let end = time::now(); //获取结束时间
    println!(
        "done!start : {:?},end :{:?},duration:{:?}",
        start,
        end,
        end - start
    );
}
