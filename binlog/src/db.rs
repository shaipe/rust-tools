use mongodb::{bson, doc, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

/// 写入mongodb
fn write_to_mongo(db_conf: &DBConfig, data: Vec<AnalyzeResult>){

    println!("正在对{:?}条数据进行入库..", data.len());

    let client = Client::connect(&db_conf.server, db_conf.port)
        .expect("Failed to initialize standalone client.");

    let coll = client.db(&db_conf.database).collection("logs");

    unsafe {
        while !COMPLETED {
            let t_q = &SQL_QUEUE;
            if !t_q.is_none(){
                // let x = SQL_QUEUE.unwrap();

                // let que: Option<Queue<AnalyzeResult>> = SQL_QUEUE;
                // let mut q = que.unwrap();
                // for _ in 0..q.len() {
                //     let mut docs: Vec<Document> = vec![];
                //     let ar = &q.pop();
                //     docs.push(ar.to_doc());
                //     // for ar in que.pop() {
                //     //     docs.push(ar.to_doc());
                //     // }

                //     // Insert document into 'test.movies' collection
                //     coll.insert_many(docs.clone(), None)
                //             .ok();
                // }
                
            }
            else{
                // 没有数据时休息200ms
                thread::sleep(std::time::Duration::from_micros(200));
            }
        }
    }
    
    // let mut docs: Vec<Document> = vec![];
    // for ar in data {
    //     docs.push(ar.to_doc());
    // }
    // // let doc = doc! {
    // //     "title": "Jaws",
    // //     "array": [ 1, 2, 3 ],
    // // };

    // // Insert document into 'test.movies' collection
    // coll.insert_many(docs.clone(), None)
    //         .ok(); //.expect("Failed to insert document.");

    // client
    // std::panic::catch_unwind(|| {
    //         println!("{}", "Failed to insert document.");
    //     }
    // );
    // // Find the document and receive a cursor
    // let mut cursor = coll.find(Some(doc.clone()), None)
    //     .ok().expect("Failed to execute find.");

    // let item = cursor.next();

    // // cursor.next() returns an Option<Result<Document>>
    // match item {
    //     Some(Ok(doc)) => match doc.get("title") {
    //         Some(&Bson::String(ref title)) => println!("{}", title),
    //         _ => panic!("Expected title to be a string!"),
    //     },
    //     Some(Err(_)) => panic!("Failed to get next from server!"),
    //     None => panic!("Server returned no results!"),
    // }
}