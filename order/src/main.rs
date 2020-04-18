
use std::collections::HashMap;
use reqwest::header;
use std::thread;

fn main() {
    let content = lane::fs::read_content("./orders.txt");
    // println!("{:?}", content);

    let ves: Vec<&str> = content.split(",").collect();

    let per_vec = lane::slice_per_vec(ves, 50);
    // println!("总计: {}", per_vec.len());
    let count = per_vec.len();
    let mut r: i32 = 1;
    for ids in per_vec {
        let mut params: HashMap<String, String> =  HashMap::new();
        params.insert("method".to_owned(), "vast.order.plan.receivingorderbyorderid".to_owned());
        params.insert("v".to_owned(), "2.0".to_owned());
        params.insert("orderid".to_owned(), ids.join(",").to_owned());
        println!("{:?}", ids.join(","));
        let x = post("Route.axd", params);
        match x {
            Ok(x) => println!("Success {:?}\n", x),
            Err(e) => println!("Error {:?}", e)
        }
        println!("第{:?}/{:?},执行完成", r, count);
        
        r += 1;

        thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("excute complete!")
}




/// 默认user_agent
const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3534.4 Safari/537.36";
// Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.5 Safari/605.1.15


/// 采用post方式请求数据
pub(crate) fn post(url: &str, params: HashMap<String, String>) -> Result<String, std::io::Error> {
    match reqwest::blocking::Client::new()
            .post(url)
            .header(header::USER_AGENT, DEFAULT_USER_AGENT)
            .form(&params)
            .send()
        {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text() {
                        Ok(txt) => Ok(txt),
                        Err(e) => Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("{:?}", e),
                        )),
                    }
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "error"))
                }
            }
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )),
        }
}