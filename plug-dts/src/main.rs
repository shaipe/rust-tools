use std::thread;
mod config;
use config::{get_config, WebConfig};
mod apporder;
use apporder::AppOrder;
pub mod accesstoken;
pub mod appauthorise;
use appauthorise::AppAuthorise;
pub mod proxy;
#[macro_use]
extern crate mysql;
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Utc};
fn main() {
    let web_confs: Vec<WebConfig> = get_config("");
    for web_conf in web_confs {
        let version_apps: Vec<i32> = web_conf.version_apps.clone();
        let fk_id: u64 = 1;
        let fk_flag: u32 = 3;
        //实例化
        let apporder = AppOrder::new(&web_conf, fk_id, fk_flag, "运营商");
        let res = apporder.async_app();
        log(&format!("同步插件返回值={:?}", res));
        //插入默认的组件
        let app_authorise = AppAuthorise::new(fk_id, fk_flag);
        log(&format!("插件{:?}正在初始化默认值", web_conf.app_id));
        let res = app_authorise.insert_default(&version_apps, web_conf.app_id);
        log(&format!("插件{:?}默认值返回值={:?}", web_conf.app_id, res));
        for version_app in version_apps {
            //总数量
            let count = apporder.get_version_count(version_app);

            let mut theads = vec![];
            let page_size = 50;
            let mut page_count = count / page_size;
            if count % page_size > 0 {
                page_count = page_count + 1;
            }
            log(&format!("版本{:?}正在初始化", version_app));
            //多线程操作
            for r in 0..page_count {
                let o = web_conf.clone();
                let h = thread::spawn(move || {
                    //println!("正在按{}分析", r);
                    log(&format!(
                        "线程{},版本{:?}正在初始化插件{:?}",
                        r, version_app, o.app_id
                    ));
                    let apporder = AppOrder::new(&o, fk_id, fk_flag, "运营商");

                    apporder.batch_insert_order(
                        page_size,
                        r,
                        version_app,
                        o.app_id,
                        &o.app_name,
                        "手动升级用户插件",
                    );
                    log(&format!("第{}线程分析执行完毕", r));
                });
                theads.push(h);
            }
            // 待待所有分析完成
            for th in theads {
                th.join().expect("thread failed");
            }
        }
    }
    log("执行完毕");
}
fn log(message: &str) {
    let time_str = {
        let time = Local::now();
        time.format("%H:%M").to_string()
    };
    println!("[{}] {}", time_str, message);
}
