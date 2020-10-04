extern crate proc;

#[proc::show_streams(4)]
fn wrapped_function(t: i32) -> i32 {
    t + 5
}

// use proc::{show_streams, AnswerFn, HelperAttr};

// #[derive(AnswerFn)]
// struct Struct;

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
    println!("{:?}", wrapped_function(6));

    // println!("{:?}", answer());
}
