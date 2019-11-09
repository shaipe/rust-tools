use std::path::Path;
use std::fs::read_dir;
use time::*;

fn main() {
    let start = time::now();//获取开始时间
    // let s = get_file_size("/Users/shaipe/downloads/files/zip/证件照.zip");
    let (s, x) = get_dir_size(Path::new("/Users/shaipe/documents"));
    println!("files: {}, size: {:?}", x, format_size(s));
    let end = time::now();//获取结束时间
    println!("done!start : {:?},end :{:?},duration:{:?}",start,end,end-start);
}

/// 获取单个件占用空间大小
/// param1: 文件路径
pub fn get_file_size(file_path: &str) -> u64 {
    let size = match Path::new(file_path).metadata() {
        Ok(metadata) => {
            metadata.len()
        }
        Err(_e) =>  0
    };
    size
}

pub fn get_dir_size(dir_path: &Path) -> (u64, u64) {
    // let p = Path::new(dir_path);
    let mut size = 0u64;
    let mut lenx = 0u64;
    if dir_path.exists() &&  dir_path.is_dir() {
        for entry in read_dir(dir_path).unwrap(){
            let path = entry.unwrap().path();
            
            // 只对当前目录下的文件进行分类处理
            if path.is_file() {
                lenx += 1;
                match path.metadata() {
                    Ok(metadata) => {
                        // println!("{:?}, {:?}", path.display(), size);
                        size += metadata.len()
                    }
                    Err(_e) =>  {}
                };
            }
            else {
                // println!("{}, {}", path.clone().display(), size);
                let (s, l) = get_dir_size(&path);
                lenx += l;
                size += s;
            }
        }
    }
    (size, lenx)
}

/// 对存储空间大小进行输出格式化
/// param1: 字节大小
pub fn format_size(size: u64) -> String {
    let k_size: f64 = 1024.0;
    let m_size: f64 = 1024.0 * 1024.0;
    let g_size: f64 = 1024.0 * 1024.0 * 1024.0;
    let t_size: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;
    let x_size = size as f64;
    let res = if x_size > t_size {
        format!("{:.2}{}", x_size / t_size, "T")
    } else if x_size > g_size {
        format!("{:.2}{}", x_size / g_size, "G")
    } else if x_size > m_size {
        format!("{:.2}{}", x_size / m_size, "M")
    } else if x_size > k_size {
        format!("{:.2}{}", x_size / k_size, "K")
    } else {
        format!("{}{}", size, "B")
    };
    res
}