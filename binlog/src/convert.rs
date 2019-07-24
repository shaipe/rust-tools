use std::path::Path;

/// binlog日志分析结构类
pub struct Convert {
    pub dir: String
}

impl Convert {

    /// 创建一个日志分析结构体
    pub fn new(dir_path: &str) -> Self {
        Convert {
            dir: dir_path.to_string()
        }
    }

    pub fn to_sql(&self) {
        let path = Path::new(self.dir.as_str());
        println!("{:?}", path.display());
    }
}

