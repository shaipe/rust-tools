#[warn(dead_code)]

use mysql::{Pool, from_row};

// 获取数据库中所有表的查询语句
// const TABLES_COMMAND_TEXT: &str = "SELECT TABLE_NAME AS Name, TABLE_COMMENT as Description,TABLE_ROWS,CREATE_TIME  FROM information_schema.tables where table_schema='{}' and table_type='base table';";

// 获取指定表中的所有列字段
// const COLUMNS_COMMAND_TEXT: &str  = "SELECT * FROM information_schema.COLUMNS where table_schema = '{db_name}' AND table_name='{table_name}';";

// 获取所有数据库名称
const DATABASE_COMMAND_TEXT: &str  = "SHOW DATABASES;";

// fn get_opts() -> Opts {
//     Opts {

//     }
// }

/// 获取数据库服务器上的所有数据库
pub fn get_databases(conn_str: &str) -> Vec<String> {
    let pool = Pool::new(conn_str).unwrap();
    let mut dbs: Vec<String> = vec![];
    for row in pool.prep_exec(DATABASE_COMMAND_TEXT, ()).unwrap() {
        let name = from_row(row.unwrap());
        dbs.push(name);
    }
    dbs
}

/// 列信息
#[derive(Debug, Clone)]
pub struct Column {
    // 列名
    pub column_name: Option<String>,
    // 数据类型
    pub data_type: Option<String>,
    // 列类型
    pub column_type: Option<String>,
    // 列名描述
    pub column_comment: Option<String>,
    // 列给定长度
    pub column_length: Option<i32>
}

/// 开发语言
pub enum Language {
    Rust,
    CSharp,
    Java
}



impl Column {
    
    /// 获取代码
    pub fn get_codes(&self, lang: Language) {
        match lang {
            // Rust语言
            Rust => {
                
            },
            _ => {}
        }
    }
}

/// 获取指定表的所有列信息
pub fn get_columns(conn_str: &str, table_name: &str, db_name: &str) -> Vec<Column> {
    let pool = Pool::new(conn_str).unwrap();
    let sql = format!("SELECT column_name, data_type, column_type, column_comment, character_maximum_length FROM information_schema.COLUMNS where table_schema = '{db_name}' AND table_name='{table_name}';", db_name=db_name, table_name=table_name);
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
