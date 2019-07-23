
/// 创建爬虫对象
pub struct Crawler {
    // 域名
    pub domain: String,
    // 基础页面
    pub base: String,
    // 已访问页面地址
    visited: Vec<String>
}


/// 爬虫对象功能实现
impl Crawler {


    /// 创建一个爬虫对象
    pub fn new(domain: String, base: String) -> Self {
        Crawler {
            domain,
            base,
            visited: vec![]
        }
    }

    /// 开始获取
    pub fn crawl(&mut self) {
        let base = self.base.clone();
        self.parse(0, &base);
    }

    /// 解析内容
    fn parse(&mut self, depth: i32, path: &String) {
        println!("{}", path);
    }
}