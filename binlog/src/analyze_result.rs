use mongodb::{bson, doc, Document};

/// binlog日志分析结果
#[derive(Clone, Debug)]
pub struct AnalyzeResult {
    // sql执行时间 
    pub execute_time: i64,
    // 表名
    pub table_name: String,
    // sql语句
    pub command_text: String,
    // 执行类型
    pub execute_type: String
}

/// 分析结果
impl AnalyzeResult {

    /// 创建一个分析结果
    // fn new(execute_time: String, table_name: String, command_text: String, execute_type: String) -> Self {
    //     AnalyzeResult{
    //         execute_time,
    //         table_name,
    //         command_text,
    //         execute_type
    //     }
    // }

    /// 将定义的结构数据转换为Beson模式下的Document
    pub fn to_doc(&self) -> Document {
        let doc = doc!{
            "execute_time" => (self.execute_time),
            "execute_type"=> (&self.execute_type),
            "table_name"=> (&self.table_name),
            "command_text"=> (&self.command_text)
        };
        doc
    }
}
