//! copyright 


use lane_mysql::MysqlConfig;

mod config;
use config::get_config;


mod apporder;
use apporder::AppOrder;

pub mod proxy;


fn main() {
    
    println!("Hello, world!");
    let web_conf: MysqlConfig = get_config("");
    let version_app: i32 = 1006;
    let free_app: i32 = 1017;
    let free_app_name = "UseSaleMode";
    let fk_id: u64 = 1;
    let fk_flag: u32 = 3;
    let apporder = AppOrder::new(fk_id, fk_flag, "运营商");
    let x = apporder.batch_insert_order(version_app, free_app, free_app_name, "手动升级用户插件");
    println!("{:?}", x);
}
