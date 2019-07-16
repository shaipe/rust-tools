

/// binlog日志分析结果
struct AnalyzeResult {
    // sql执行时间 
    execute_time: String,
    // 表名
    table_name: String,
    // sql语句
    command_text: String,
    // 执行类型
    excute_type: String
}



fn main() {
    println!("Hello, world!");
}
