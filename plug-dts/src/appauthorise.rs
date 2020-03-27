
use lane_mysql::{DBValue, Table};

use std::collections::HashMap;
use std::io::Error;
use mysql::*;
pub struct AppAuthorise {
    pub fk_id: u64,
    pub fk_flag: u32,
}
impl Table for AppAuthorise {
    // 实现表名
    fn get_table_name(&self) -> String {
        "pak_AppAuthorise".to_owned()
    }
}
impl AppAuthorise {
    /**
     * 构造函数
     */
    pub fn new(_fk_id: u64, _fk_flag: u32) -> Self {
        AppAuthorise {
            fk_id: _fk_id,
            fk_flag: _fk_flag,
        }
    }
    pub fn update_aync_state(&self){
        let param: Vec<(String, mysql::Value)> = params! {
            "is_sync" => false
        };
        let res = self.update("IsSync=:is_sync", &format!("FKId={} and FKFlag={}", self.fk_id,self.fk_flag), param);
        println!("更新同步状态{:?}", res);
        //Ok(0)
    }
}
