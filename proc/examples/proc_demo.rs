extern crate proc;

#[proc::show_streams(4)]
fn wrapped_function(t: i32) -> i32 {
    t + 5
}

use proc::{show_streams, AnswerFn};

#[derive(AnswerFn, Debug, Clone)]
struct StructT{
    id: i32,
    sss_len: i32,
}

// #[derive(HelperAttr)]
// struct Struct2 {
//     #[helper]
//     field: (),
// }

// // Example: Basic function
// #[show_streams]
// fn invoke1() {}
// // out: attr: ""
// // out: item: "fn invoke1() { }"

// // Example: Attribute with input
// #[show_streams(bar)]
// fn invoke2() {}
// // out: attr: "bar"
// // out: item: "fn invoke2() {}"

// // Example: Multiple tokens in the input
// #[show_streams(multiple => tokens)]
// fn invoke3() {}
// // out: attr: "multiple => tokens"
// // out: item: "fn invoke3() {}"

// // Example:
// #[show_streams { delimiters }]
// fn invoke4() {}
// // out: attr: "delimiters"
// // out: item: "fn invoke4() {}"

fn main() {
    // println!("{:?}", wrapped_function(6));

    println!("{:?}", StructT{
        id: 0,
        sss_len: 0,
    });

    // println!("{:?}", answer());
}
