/**
 * 数据库对象
 */

use mysql::{Pool, from_row};
use crate::table::Table;

/// 数据库对象结构
#[derive(Debug, Clone)]
pub struct Database {
    pub name: String,
    pub conn_str: String,
    conn: Pool
}


impl Database {

    /// 创建一个数据库对象
    pub fn new(name: &str, conn_str: &str) -> Self {
        Database{
            name: name.to_owned(),
            conn_str: conn_str.to_owned(),
            conn: Pool::new(conn_str).unwrap()
        }
    }

    #[allow(dead_code)]
    pub fn get_conn(&self) -> Pool{
        self.conn.clone()
    }

    #[allow(dead_code)]
    pub fn get_tables(&self) -> Vec<String> {
        vec![]
    }


    pub fn get_talbe(&self, table_name: &str) -> Table {
        Table::new(&self.name, table_name, self.conn.clone())
    }
}


/// 获取数据库服务器上的所有数据库
#[allow(dead_code)]
pub fn get_databases(conn_str: &str) -> Vec<String> {
    // 获取所有数据库名称
    let dbase_command_text: &str  = "SHOW DATABASES;";
    let pool = Pool::new(conn_str).unwrap();
    let mut dbs: Vec<String> = vec![];
    for row in pool.prep_exec(dbase_command_text, ()).unwrap() {
        let name = from_row(row.unwrap());
        dbs.push(name);
    }
    dbs
}