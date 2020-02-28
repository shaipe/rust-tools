
mod config;
use lane_mysql::{MysqlConfig};
use config::{get_config};
mod apporder;
use apporder::AppOrder;
pub mod prox;
pub mod accesstoken;
fn main() {
    println!("Hello, world!");
    let web_conf: MysqlConfig = get_config("");
    let version_app:i32=1006;
    let free_app:i32=204;
    let free_app_name="UseSaleMode";
    let fk_id:u64=1;
    let fk_flag:u32=3;
    let apporder=AppOrder::new(web_conf.db_id,fk_id,fk_flag,"运营商");
    apporder.batch_insert_order(version_app, free_app, free_app_name, "手动升级用户插件");
    //println!("{:?}",x);

   

}
