/**
 * 数据库表处理
 */

use crate::column::Column;
// use crate::dbase::Database;
use mysql::{Pool, from_row};

/// 表对象
#[derive(Debug, Clone)]
pub struct Table {
    pub db_name: String,
    pub name: String,
    pub conn: Pool 
}

impl Table {

    /// 创建一个表对象
    pub fn new(db_name: &str, name: &str, pool: Pool) -> Self {
        Table{
            db_name: db_name.to_owned(),
            name: name.to_owned(),
            conn: pool
        }
    }

    pub fn get_columns(&self) -> Vec<Column> {
        let pool = self.conn.clone(); // Pool::new(conn_str).unwrap();
        let sql = format!("SELECT column_name, data_type, column_type, column_comment, character_maximum_length FROM information_schema.COLUMNS where table_schema = '{db_name}' AND table_name='{table_name}';", db_name=self.db_name, table_name=self.name);
        // let mut columns: Vec<Column> = vec![];
        // println!("{}", sql);
        // for row in pool.prep_exec(sql, ()).unwrap() {
        //     let (column_name, data_type, column_type, column_comment, character_maximum_length) = from_row(row.unwrap());
        //     let col = Column {
        //         column_name: column_name,
        //         data_type: data_type,
        //         column_type: column_type,
        //         column_comment: column_comment,
        //         column_length: character_maximum_length,
        //     };
        //     columns.push(col);
        // }
        let columns: Vec<Column> = pool.prep_exec(sql, ())
            .map(|result| {
                result.map(|x| x.unwrap())
                .map(|row| {
                    let (column_name, data_type, column_type, column_comment, character_maximum_length) = from_row(row);
                    Column {
                        column_name: column_name,
                        data_type: data_type,
                        column_type: column_type,
                        column_comment: column_comment,
                        column_length: character_maximum_length,
                    }
                }).collect()
            }).unwrap();
        columns
    }
}