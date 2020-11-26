use chrono::Local;
use log::{error, info};
use std::error::Error;

mod config;
use config::{get_config, RequestConfig};

// /// 添加数据测试
// async fn add_data(times: u64) {
//     // let data = r#"{"author":"X项目研发组","category":"类别2","createdBy":"X项目组",
//     // "createdTime":"2020-11-18T07:49:33.306Z","docPath":"",
//     // "hits":0,"overt":true,"summary":"","tags":"tag1，tag2","title":"压力测试数据标题"}"#;

//     // let json_value: serde_json::Value = serde_json::from_str(&data).unwrap();

//     match reqwest::Client::new()
//         .post("http://192.168.17.212:8083/demo/document")
//         .json(&serde_json::json!({
//             "author": "X项目组",
//             "category":"类别2",
//             "createdTime": Local::now().to_rfc3339(),
//             "docPath": "",
//             "hits": "0",
//             "overt": "true",
//             "summary": "X项目组L0底层框架测试内容。拆分服务的颗粒度应该根据业务发展和团队现状综合去考虑。这里可以参考一个很火的理论「 康威定律 」。什么样的团队，就产生什么样的架构，微服务拆分的颗粒度是需要和团队结构相匹配的。当你着手拆微服务的时候，得先评估一下团队人员和素质，一般在开发期，2-3个人开发一个服务是合理的，在维护期，1个人维护2-3个服务也是合理的。",
//             "tags": "tag1,tag2,tags",
//             "title": format!("X项目组底层压力测试-{}",times)
//         }))
//         .send()
//         .await
//     {
//         Ok(res) => {
//             // 成功
//             if res.status() == 200 {
//                 // 返回状态成功
//                 // println!("success {}", times);
//             } else {
//                 println!("error");
//                 // 返回错误
//             }
//         }
//         Err(err) => {
//             // 报错
//             println!("{}", err);
//         }
//     }
// }

/// 用GET方式获取数据
async fn get_data(conf: RequestConfig) {
    match reqwest::Client::new().get(&conf.url).send().await {
        Ok(res) => {
            // 成功
            if res.status() == 200 {
                // 返回状态成功
                match res.text().await {
                    Ok(txt) => info!("{}", txt),
                    Err(e) => println!("{}", e),
                }
            } else {
                error!("error {}", res.status());
                println!("{:?}", "error");
                // 返回错误
            }
        }
        Err(err) => {

            // 报错
            error!("{}", err);
        }
    }
}

/// Post数据
async fn post_data(times: u64, conf: RequestConfig) {
    // 对数据进行处理
    let data = conf
        .data
        .replace("{{times}}", &times.to_string())
        .replace("{{add_time}}", &Local::now().to_rfc3339());

    // 将字符串转换为Json数据
    let json_value: serde_json::Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            println!("json data error {:?} :::\n {}", e, data);
            serde_json::json!({})
        }
    };

    // Post数据到接口文件
    match reqwest::Client::new()
        .post(&conf.url)
        .json(&json_value)
        .send()
        .await
    {
        Ok(res) => {
            // 成功
            if res.status() == 200 {
                // 返回状态成功
                // println!("success {}", times);
            } else {
                error!("error {}", res.status());
                // 返回错误
            }
        }
        Err(err) => {
            // 报错
            println!("{}", err);
        }
    }
}

/// 模拟用户数量
async fn execute_time(current_id: u64, delay: u64, req_conf: RequestConfig) {
    // 记录启动时间
    let start = std::time::Instant::now();
    // 定义执行时长
    let ten_millis = std::time::Duration::from_millis(delay * 1000);
    loop {
        match req_conf.method.to_uppercase().as_str() {
            "GET" => {
                watch_time!(&req_conf.name, get_data(req_conf.clone()).await);
            }
            "PUT" => {
                info!("还未支持PUT方法");
            }
            "DELETE" => {
                info!("还未支持DELETE方法");
            }
            _ => {
                watch_time!(&req_conf.name, post_data(current_id, req_conf.clone()).await);
            }
        }

        // 如果给定了执行时间就执行时间到了后结束
        if delay > 0 {
            let now = std::time::Instant::now();
            // println!("{:?}", now);
            if now - start > ten_millis {
                break;
            }
        } else {
            // 未给定执行时间就直接退出
            break;
        }
    }
}

/// 入口方法
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // 获取启动输入参数
    let args: Vec<String> = std::env::args().collect();

    // 指定要执行的名称
    let arg1 = if args.len() > 1 { &args[1] } else { "" };

    // 获取请求配置项
    let current_req_config: Option<RequestConfig> = get_config("", arg1);

    if let Some(conf) = current_req_config {
        // 指定用户数
        let arg2 = if args.len() > 2 {
            args[2].parse::<u64>().unwrap()
        } else {
            1
        };

        // 指定执行时长
        let arg3 = if args.len() > 3 {
            args[3].parse::<u64>().unwrap()
        } else {
            0
        };

        let log_path = format!("reqest_{}.log", conf.name);

        // 日志初始化
        let _ = fast_log::init_log(&log_path, 1000, log::Level::Info, None, false);

        // 创建模拟用户数
        let mut tasks = vec![];
        for i in 0..arg2 {
            let tc = conf.clone();
            tasks.push(tokio::task::spawn(async move {
                let c = tc.clone();
                execute_time(i, arg3, c).await;
            }));
        }

        // 开始执行
        for t in tasks {
            let _ = t.await;
        }
    } else {
        println!("配置文件错误, 没有找到名称为：{}的配置信息", arg1)
    }

    Ok(())
}

/// ## Usage
/// rust
/// use simple_timer::watch_time;
///
/// fn hello_world() {
///     println!("hello world");
/// }
///
/// fn main() {
///     watch_time!("time_1", hello_world());
///     watch_time!("time_two",
///         {
///             println!("great weather");
///             println!("i agree");
///         }
///     );
/// }
///
#[macro_export]
macro_rules! watch_time {
    ($t: expr, $x:expr) => {{
        use std::time::Instant;
        let start = Instant::now();
        let res = $x;
        let end = start.elapsed();
        info!(
            "excute({})use time, {}.{:03}",
            $t,
            end.as_secs(),
            end.subsec_millis()
        );
        res
    }};
}
