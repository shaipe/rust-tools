extern crate libc;

use std::{thread, str};
//external crates
// use std::os::raw::c_char;
use std::ffi::CString;
use libc::{c_char};
use std::ffi::CStr;

/// 性能测试
#[no_mangle]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub extern fn process(){
    let handles :Vec<_> =(0..100).map(|_|{
        thread::spawn(||{
            let mut x= 0;
            for _ in 0..500000000 {
                x+=1
            }
            x
        })
    }).collect();

    let xc = handles.len();

    for h in handles{
        println!("Thread finished with count={}",h.join().map_err(|_| "Could not join a thread!").unwrap());
    }

    println!("done!{:?}", xc);
}

/// 字符串输入输出测试,但未实现
#[no_mangle]
pub extern fn test(text: &str) -> String {
    println!("xxx === {}", text);
    String::from(text)
}

/// 传入数字int型测试
#[no_mangle]
pub extern fn add_numbers(number1: i32, number2: i32) -> i32 {
    println!("Hello from rust!");
    number1 + number2
}

/// 结构体输出
#[repr(C)]
pub struct SampleStruct {    
    pub field_one: i16,
    pub field_two: i32,
}

/// 获取一个简单的结构体
#[no_mangle]
pub extern fn get_simple_struct() -> SampleStruct {
    SampleStruct {
        field_one: 1,
        field_two: 2
    }
}

/// 输出字符串测试
#[no_mangle]
pub extern fn string_from_rust(s: *const c_char) -> *mut c_char {
    let c_value = unsafe { CStr::from_ptr(s).to_bytes() };
    let r_str = match str::from_utf8(c_value) {
        Ok(value) => {
            value
        },
        Err(_) => "-1",
    };
    println!("urst {}", r_str);
    let c_str_song = CString::new(r_str).unwrap();
    c_str_song.into_raw()
    // let r_str = std::str::from_utf8(c_str.to_bytes()).unwrap();
    // println!("{}", r_str);
    // let mut s = String::from("Hello World ::: ");
    // s.push_str(r_str);
    // s
}


/// 传入字字符串
#[no_mangle]
pub extern fn printc(s: *const c_char) {
    let c_str : &CStr = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    println!("{:?}", c_str.to_bytes().len()); //prints "1" if unicode

    let r_str = std::str::from_utf8(c_str.to_bytes()).unwrap();
    println!("{:?}", r_str);
}

#[repr(C)]
pub struct test {
 pub isbool: bool,
}

#[no_mangle]
pub extern fn how_many_characters(s: *const c_char) -> u32 {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as u32
}


#[no_mangle]
pub extern "C" fn count_substrings(value: *const c_char, substr: *const c_char) -> i32 {
    let c_value = unsafe { CStr::from_ptr(value).to_bytes() };
    let c_substr = unsafe { CStr::from_ptr(substr).to_bytes() };
    match str::from_utf8(c_value) {
        Ok(value) => match str::from_utf8(c_substr) {
            Ok(substr) => rust_substrings(value, substr),
            Err(_) => -1,
        },
        Err(_) => -1,
    }
}

fn rust_substrings(value: &str, substr: &str) -> i32 {
    println!("arg1 {}, arg2 {}", value, substr);
    value.matches(substr).count() as i32
}