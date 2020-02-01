//! copyrigt
//! postgres数据库的实现
//! 


 
use crate::Sql;

pub struct Postgres;

impl Postgres {
    pub fn new() -> Self {
        Postgres {}
    }
}

impl Sql for Postgres {

    /// 荛取数据类型
    fn get_data_type(&self, type_str: &str) -> String{
        // println!("{}", type_str);
        match type_str.to_lowercase().trim() {
            "i8" => "SMALLINT".to_owned(),
            "i64" | "i32" => "INT".to_owned(),
            "f64" | "f32" => "FLOAT(26,6)".to_owned(),
            "datetime<local>" | "datetime<utc>" | "naivedatetime" => "DATETIME".to_owned(),
            "naivedate" => "DATE".to_owned(),
            _ => "VARCHAR(50)".to_owned()
        }
    }

    fn get_identifier(&self) -> String {
        "  id serial primary key,".to_owned()
    }

    /// 根据文件内容获取创建表的sql语句
    fn get_sql(&self, content: String, prefix: &str) -> String {
        let lines: Vec<&str> = content.split("\n").collect();

        let mut structed = false;
        let mut res: Vec<String> = vec![];

        for line in lines {
            if line == "" {
                continue;
            } else {
                if line.contains("struct ") {
                    structed = true;
                    let y = line.replace("pub struct ", "").replace("{", "");
                    let mut yx = String::new();
                    yx.push_str(prefix);
                    yx.push_str(&y.trim().to_lowercase());
                    // println!("{:?}",yx);
                    res.push(format!("-- table {} ", yx));
                    res.push(format!("CREATE TABLE {} (", yx));
                    res.push(self.get_identifier());
                } else {
                    if structed {
                        if line == "}" {
                            // res.push("  PRIMARY KEY (id)".to_owned());
                            res.push(");\n\n".to_owned());
                            structed = false;
                        } else if line.contains(":") {
                            let l = line.replace("pub ", "");
                            let f: Vec<&str> = l.split(":").collect();
                            // 如果有字段的名字为id跳过,已经为表添加了默认的id字段
                            if f[0].trim().to_lowercase().as_str() == "id" {
                                continue;
                            }
                            let x: Vec<&str> = f[1].split(",").collect();

                            // let comment = if x.len() > 1 {
                            //     x[1].replace("//", "")
                            // } else {
                            //     "".to_owned()
                            // };

                            let lf: String = format!(
                                "  {} {} DEFAULT NULL,",
                                f[0].trim(),
                                self.get_data_type(x[0])
                            );
                            res.push(lf);
                        }
                    }
                }
            }
            // println!("{}", line);
        }
        res.join("\n")
    }
}

