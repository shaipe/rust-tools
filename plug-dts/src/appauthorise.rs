use lane_mysql::{DBValue, Table};

use mysql::error::Error;
use std::collections::HashMap;
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
    pub fn update_aync_state(&self) {
        let param: Vec<(String, mysql::Value)> = params! {
            "is_sync" => false
        };
        let res = self.update(
            "IsSync=:is_sync",
            &format!("FKId={} and FKFlag={}", self.fk_id, self.fk_flag),
            param,
        );
        println!("更新同步状态{:?}", res);
        //Ok(0)
    }
    /**
     * 获得版本对应的供应商数据
     */
    pub fn insert_default(
        &self,
        version_apps: &Vec<i32>,
        app_id: i32,
    ) -> Result<u64, mysql::error::Error> {
        // let  version_app_ids:Vec<mysql::Value>=version_apps.iter().map(|x|{
        //     mysql::Value::from(x)
        // }).collect();
        let version_app_ids: Vec<String> = version_apps.iter().map(|x| x.to_string()).collect();
        let param: Vec<(String, mysql::Value)> = params! {
            "app_id" => app_id
        };

        let sql = format!(
            r#"
        INSERT INTO pak_appauthorise(FKId,FKFlag,AppId,NAME,Icon,Description,Unit,Price,TimePeriod,PeriodUnit,Radix,KeyWord,Sells,Reorder,STATUS,Users,AppType,BuyType,VerReorder,IsSync)
        SELECT a.FKId,a.FKFlag,c.Id,c.NAME,Icon,c.Description,c.Unit,c.Price,c.TimePeriod,c.PeriodUnit,Radix,c.KeyWord,c.Sells,c.Reorder,1,1,c.AppType,c.BuyType,c.VerReorder,1
            FROM pak_customerapp AS a
          , sup_supplier AS b 
          , pak_app AS c
          WHERE a.fkid=b.id AND a.fkflag=2 and b.expireTime>NOW() AND a.appid not IN({}) AND c.Id=:app_id;
        "#,
            version_app_ids.join(",")
        );
        println!("{:?}", sql);
        let pool = self.get_pool();
        let mut stmt = match pool.prepare(sql) {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("{:?}", e);
                return Err(e);
            }
        };
        let res = stmt.execute(param);
        // 返回最后插入的id
        let c = match res {
            Ok(r) => Ok(r.affected_rows()),
            Err(e) => Err(e),
        };
        c
        //return Ok(res);
    }
}
