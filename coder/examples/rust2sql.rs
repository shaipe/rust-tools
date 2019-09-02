/**
 * 根据rust代码中的struct结构转换成sql表生成文件
 * create by shaipe
 * create time 20190901
 */
use lane::fs::{read_content, append_content};
use std::path::Path;

fn main() {

    let dir_path = Path::new("/Users/shaipe/workspace/rust/sst/sst-stock/src/");

    let files: Vec<&str> = vec![
        "stock.rs",
        "hist.rs",
        "real.rs"
    ];

    for f in files {
        let file_path = dir_path.join(f);
        let file_str = format!("{}", file_path.display());
        println!("{}", file_str);
        let content = read_content(&file_str);
        
        let sql = get_create_sql(content);
        append_content("/Users/shaipe/workspace/rust/sst/resources/init.sql", sql.as_bytes());
        // println!("{}", sql);
    }
    
    
}

fn get_create_sql(content: String) -> String {

    let lines: Vec<&str> = content.split("\n").collect();

    let mut structed = false;
    let mut res: Vec<String> = vec![];
    
    for line in lines {
        if line == "" {
            continue;
        }
        else{
            if line.contains("struct ") {
                structed = true;
                let y = line.replace("pub struct ", "").replace("{", "");
                let yx = y.trim().to_lowercase();
                // println!("{:?}",yx);
                res.push(format!("-- table {} ", yx));
                res.push(format!("CREATE TABLE `{}` (", yx));
                res.push("  `id` INT(11) NOT NULL AUTO_INCREMENT,".to_owned());
            }
            else{
                if structed {
                    if line == "}"{
                        res.push("  PRIMARY KEY (`id`)".to_owned());
                        res.push(") ENGINE=InnoDB DEFAULT CHARSET=utf8;\n\n".to_owned());
                        structed = false;
                    }
                    else if line.contains(":"){
                        let l = line.replace("pub ", "");
                        let f: Vec<&str> = l.split(":").collect();
                        let x: Vec<&str> = f[1].split(",").collect();

                        let comment = if x.len() > 1 {
                            x[1].replace("//", "")
                        }
                        else {
                            "".to_owned()
                        };

                        let lf: String = format!("  `{}` {} DEFAULT NULL COMMENT '{}',", f[0].trim(), get_data_type(x[0]), comment.trim());
                        res.push(lf);
                    }
                }
            }
        }
        // println!("{}", line);
    }
    res.join("\n")
}


fn get_data_type(type_str: &str) -> String{
    // println!("{}", type_str);
    match type_str.to_lowercase().trim() {
        "i64" | "i32" => "INT(11)".to_owned(),
        "f64" | "f32" => "FLOAT(12,6)".to_owned(),
        "datetime<local>" | "datetime<utc>" => "DateTime".to_owned(),
        _ => "VARCHAR(50)".to_owned()
    }
}