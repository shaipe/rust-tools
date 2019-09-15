/**
 * 根据rust代码中的struct结构转换成sql表生成文件
 * create by shaipe
 * create time 20190901
 */
use std::env;
use lane::fs::{read_content, append_content};
use std::path::{Path, PathBuf};

/// 启动函数
fn main() {
    // 获取输入参数
    let args: Vec<String> = env::args().collect();
    
    let prefix = if args.len() > 1 {
        // let mut t = String::from(&args[2]);
        // t.push_str("_");
        // t
        &args[1]
    }
    else{
        ""
    };

    let dir_str = if args.len() > 2 {
        &args[2]
    }
    else{
        "./"
    };

    // 获取目录路径
    let dir_path = Path::new(dir_str);
    let file_path = dir_path.join("./tables.sql");
    let file_path_str = format!("{}", file_path.display());

    println!("开始对 {:?} 目录进行处理, 将把结果写入 {:?} 文件中.", dir_path.display(), file_path.display());

    walk(PathBuf::from(dir_path), &file_path_str, prefix);

    println!("代码生成已经完成");
    // let files: Vec<&str> = vec![
    //     "stock.rs",
    //     "hist.rs",
    //     "real.rs"
    // ];

    // for f in files {
    //     let file_path = dir_path.join(f);
    //     let file_str = format!("{}", file_path.display());
    //     println!("{}", file_str);
    //     let content = read_content(&file_str);
        
    //     let sql = get_create_sql(content);
    //     append_content("/Users/shaipe/workspace/rust/sst/resources/init.sql", sql.as_bytes());
    //     // println!("{}", sql);
    // }
    
    
}

/// 遍历整个目录对指定的rs文件进行处理
fn walk(dir_path: PathBuf, file_path: &str, prefix: &str) {
    if dir_path.exists() {
        for entry in dir_path.read_dir().expect("指定的路径 {} 不存在.") {
            let p = entry.unwrap().path();
            // println!("{:?}", p);
            if p.is_file() {
                let ext = p.extension();
                // 判断只处理后缀为rs的文件
                if !ext.is_none() && ext.unwrap() == "rs" {
                    // 获取路径的字符
                    let p_str = format!("{}", p.display());
                    // 判断只处理src目录下的文件
                    if p_str.contains(r"/src/"){
                        let content = read_content(&p_str);
                        let sql = get_create_sql(content, prefix);
                        append_content(file_path, sql.as_bytes());
                    }
                }
            }
            else{
                walk(p, file_path, prefix);
            }
        }
    }
    else{
        println!("指定的路径 {} 不存在.", dir_path.display());
    }
}

/// 根据文件内容获取创建表的sql语句
fn get_create_sql(content: String, prefix: &str) -> String {

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
                let mut yx = String::new();
                yx.push_str(prefix);
                yx.push_str(&y.trim().to_lowercase());
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
                        // 如果有字段的名字为id跳过,已经为表添加了默认的id字段 
                        if f[0].trim().to_lowercase().as_str() == "id" {
                            continue;
                        }
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

/// 荛取数据类型
fn get_data_type(type_str: &str) -> String{
    // println!("{}", type_str);
    match type_str.to_lowercase().trim() {
        "i8" => "SMALLINT".to_owned(),
        "i64" | "i32" => "INT(11)".to_owned(),
        "f64" | "f32" => "FLOAT(12,6)".to_owned(),
        "datetime<local>" | "datetime<utc>" | "naivedatetime" => "DATETIME".to_owned(),
        "naivedate" => "DATE".to_owned(),
        _ => "VARCHAR(50)".to_owned()
    }
}