use chrono::{Local, NaiveDateTime};
use lane::{get_map_value, ImplString,err_info};
use lane_mysql::{DBValue, Table};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;

use crate::prox::post;
use md5;
const DEFAULT_DTS_URL: &'static str = "http://127.0.0.1:8090";
pub struct AppOrder{
    pub fk_id:u64,
    pub fk_flag:u32,
    pub user_name: String,
    pub access_token:String
}
/// 基于Table的实现
impl Table for AppOrder {
    // 实现表名
    fn get_table_name(&self) -> String {
        "pak_customerapp".to_owned()
    }
}
impl AppOrder{
    pub fn new(_fk_id:u64,_fk_flag:u32,_user_name:&str)->Self{
        AppOrder{
            fk_id: _fk_id,
            fk_flag: _fk_flag,
            user_name: _user_name.to_owned(),
            access_token:String::new(), 
        }
    }
    pub fn get_list_version(&self,version_app:i32)->Result<Vec<(u64,i32,String)>,Error>{
        let sql=format!(r#"
        SELECT a.FKId,a.FKFlag,2 AS RunWay,b.Name AS CompanyName
FROM pak_customerapp AS a
JOIN sup_supplier AS b
WHERE a.fkid=b.id AND a.fkflag=2 AND appid={};
        "#,version_app);
        let pool = self.get_pool();
        let res: Vec<(u64,i32, String)> = pool
        .prep_exec(sql, ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| (row["FKId"].to_u64(),row["FKFlag"].to_i32(), DBValue::to_string(&row["CompanyName"], false)))
                .collect()
        })
        .unwrap();
        return Ok(res);
    }
    pub fn batch_insert_order(&self,version_app:i32,app_id:i32,app_name:&str,content:&str)->Result<Vec<(i64,String)>,std::io::Error>{
            //1、查询该版本下的供应商
        let result_list=self.get_list_version(version_app);
        let decorate_list=match result_list{
            Ok(list)=>{
                
                let mut vec_list=Vec::new();
                for item in list{
                    let name=item.2;
                    let fkid=item.0;
                    let fkflag=item.1;
                    let mut dic=HashMap::new();
                    dic.insert("Method".to_owned(), "aus.package.app.submit".to_owned());
                    dic.insert("V".to_owned(), String::from("2.0"));
                    dic.insert(String::from("Token"), self.access_token.clone());
                    dic.insert(String::from("Md5"),format!("{:x}",md5::compute(self.access_token.as_bytes())));
                    dic.insert("Flag".to_owned(), format!("Upgrade_Plug_{}",app_name));
                    dic.insert("AppId".to_owned(), format!("{}",app_id));
                    dic.insert("AppliedId".to_owned(), "0".to_owned());
                    dic.insert("Content".to_owned(), content.to_owned());
                    dic.insert("ReceiveFKId".to_owned(), format!("{}",self.fk_id));
                    dic.insert("ReceiveFKFlag".to_owned(), format!("{}",self.fk_flag));
                    dic.insert("Remark".to_owned(), format!("{}手动批量更新【操作人：任我行科技销售中心;IP=127.0.0.1】",content));
                    dic.insert("RunWay".to_owned(), "Wholesale".to_owned());
                    dic.insert("FKId".to_owned(), format!("{}",fkid));
                    dic.insert("FKFlag".to_owned(), format!("{}",fkflag));
                    dic.insert("CompanyName".to_owned(), name);
                    vec_list.push(dic);
                }
                Ok(vec_list)
            }
            Err(e)=>Err(err_info!(format!("{}", e))),
        };
         //2、同步dts中
        let x:Result<Vec<(i64,String)>,std::io::Error>=match decorate_list{
            Ok(list)=>{
                let mut error_list:Vec<(i64,String)>=Vec::new();
                for item in list{
                    let res =post(DEFAULT_DTS_URL,item.clone());
                    let fkid=item.get("FKId").unwrap().parse::<i64>().unwrap();
                    println!("{:?},{:?}",item,res);
                    match res{
                        Ok(_)=>{},
                        Err(_)=>error_list.push((fkid,"mmm".to_owned()))
                    }
                }
                Ok(error_list)
            },
            Err(e)=>Err(err_info!(format!("{:?}", e))),
        };
        x
    }
}

