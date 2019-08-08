
/// 重复类型
enum RepeatType {
    year,
    month,
    day,
    hour,
    minute,
    second
}

/// 任务
struct Task {
    // 任务名称
    name: String,
    // 任务类型
    type: String,
    // 执行时间
    execute_time: Option<String>,
    // 时间间隔
    interval: Option<i32>,
    // 重复方式
    repeat: Option<RepeatType>,
    // 请求任务
    request: Option<RequestTask>
}

/// 网络请求任务
struct RequestTask {
    // 请求路径
    url: String,
    // 请求数据
    data: String,
    // 请求方式
    method: String,
}

/// 基础配置信息
struct Config {

    name: String,

    tasks: Vec<Task>
}

impl Config {
    // 创建Config并加载相应信息
    pub fn new(conf_path: &str) -> Self {
        // println!("{}", conf_path);

        // 判断传入的conf_path是否为空,如果为空就给定默认值 
        let file_path = if conf_path.len() == 0 {
            "config.toml"
        } else {
            conf_path
        };

        // 打开文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e)
        };

        // 读取文件到字符串变量
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e)
        };
       
        // println!("file path {:?}", file_path);

        // if file_path.ends_with(".json") {

        // }
        // else {
        //     // 使用toml载加配置信息到结构体中
        //     let tc: Config = toml::from_str(&str_val).unwrap();
        //         Self {
        //         title: tc.title,
        //         server: tc.server,
        //         proxy: tc.proxy
        //     }
        // }
        let jc: Config = serde_json::from_str(&str_val).unwrap();
        jc
    }
}