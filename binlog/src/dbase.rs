use mongodb::{ Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::{ThreadedDatabase};
use mongodb::coll::Collection;
use crate::analyze_result::AnalyzeResult;
use crate::config::DBConfig;

pub struct DBase {
    pub client: Client,
    pub collection: Collection
}

impl DBase {

    pub fn new(db_conf: DBConfig) -> Self {
        let client = Client::connect(&db_conf.server, db_conf.port)
            .expect("Failed to initialize standalone client.");

        let coll = client.db(&db_conf.database).collection("logs");

        DBase{
            client: client,
            collection: coll
        }
    }

    pub fn insert(&self, data: Vec<AnalyzeResult>) {

        let mut docs: Vec<Document> = vec![];
                    
        for ar in data {
            // let ar = &q.pop();
            docs.push(ar.to_doc());
        }
                    
        // Insert document into 'test.movies' collection
        self.collection.insert_many(docs.clone(), None)
                .ok();
    }
}
