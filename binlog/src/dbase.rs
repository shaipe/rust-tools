use mongodb::{ Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::{ThreadedDatabase};
// use mongodb::coll::Collection;
use crate::analyze_result::AnalyzeResult;
use crate::config::DBConfig;

#[derive(Clone)]
pub struct DBase {
    // mongodb 客户端
    pub client: Client,
    // // 数据集
    // pub collection: Collection,

    pub db_config: DBConfig,
}

impl DBase {

    /// 创建一个一个数据库连接
    pub fn new(db_conf: DBConfig) -> Self {
        let client = Client::connect(&db_conf.server, db_conf.port)
            .expect("Failed to initialize standalone client.");

        DBase{
            db_config: db_conf,
            client: client
        }
    }

    /// 数据插入
    pub fn insert(&self, data: Vec<AnalyzeResult>) {
        println!("正在对 {:?} 条数据进行入库处理.", data.len());
        // let db_c = &self.db_config;
        // let client = Client::connect(&db_c.server, db_c.port)
        //     .expect("Failed to initialize standalone client.");

        // let coll = client.db(&db_c.database).collection("logs");

        let mut docs: Vec<Document> = vec![];

        let coll = self.client.db(&self.db_config.database).collection("logs");
                    
        for ar in data {
            // let ar = &q.pop();
            docs.push(ar.to_doc());
        }
                    
        // Insert document into 'test.movies' collection
        coll.insert_many(docs.clone(), None)
                .ok();
    }
}
