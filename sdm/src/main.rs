// mod tar;

// mod model;

// fn main() {
//     println!("Hello, world!");
// }

// use std::fmt::Display;

trait Example {
    fn call(&self);
}

impl<T> Example for T {
    fn call(&self) {
        println!("most generic");
    }
}

// impl<T> Example for T
//     where T: Display
// {
//     fn call(&self) {
//         println!("generic for Display, {}", self);
//     }
// }

impl Example for str {
    fn call(&self) {
        println!("specialized for str, {}", self);
    }
}

// fn main() {
//     let v1 = vec![1i32,2,3];
//     let v2 = 1_i32;
//     let v3 = "hello";

//     v1.call();
//     v2.call();
//     v3.call();
// }

// 一个具体类型 `A`。
struct A;

// 在定义类型 `Single` 时，第一次使用类型 `A` 之前没有写 `<A>`。
// 因此，`Single` 是个具体类型，`A` 取上面的定义。
struct Single(A);
//            ^ 这里是 `Single` 对类型 `A` 的第一次使用。

// 此处 `<T>` 在第一次使用 `T` 前出现，所以 `SingleGen` 是一个泛型类型。
// 因为 `T` 是泛型的，所以它可以是任何类型，包括在上面定义的具体类型 `A`。
struct SingleGen<T>(T);

// fn main() {
//     // `Single` 是具体类型，并且显式地使用类型 `A`。
//     let _s = Single(A);

//     // 创建一个 `SingleGen<char>` 类型的变量 `_char`，并令其值为 `SingleGen('a')`
//     // 这里的 `SingleGen` 的类型参数是显式指定的。
//     let _char: SingleGen<char> = SingleGen('a');

//     // `SingleGen` 的类型参数也可以隐式地指定。
//     let _t    = SingleGen(A); // 使用在上面定义的 `A`。
//     let _i32  = SingleGen(6); // 使用 `i32` 类型。
//     let _char = SingleGen('a'); // 使用 `char`。
// }

trait DataModel<T> {
    fn parent(&self) -> Option<T>;
}

// #[derive(Debug)]
// struct Model<T>(T);

#[derive(Debug)]
struct Demo {}

impl<T> DataModel<T> for Demo {
    fn parent(&self) -> Option<T> {
        None
    }
}

// impl<T> DataModel<T> for T {
//     fn parent(&self) -> Option<Model<T>> {
//         None
//     }
// }

#[derive(Debug)]
struct Test<Demo> {
    parent: Demo,
}

impl DataModel<Demo> for Test<Demo> {
    fn parent(&self) -> Option<Demo> {
        Some(Demo {})
    }
}

impl std::default::Default for Test<Demo> {
    fn default() -> Self {
        Test { parent: Demo {} }
    }
}

struct Test1<Test> {
    parent: Test,
}

impl std::default::Default for Test1<Test<Demo>> {
    fn default() -> Self {
        Test1 {
            parent: Test::default(),
        }
    }
}

impl DataModel<Test<Demo>> for Test1<Test<Demo>> {
    fn parent(&self) -> Option<Test<Demo>> {
        Some(Test::default())
    }
}

// type DT = DataModel<T>;

// fn get_parent<T>(c: dyn DataModel<T> + 'static) -> DataModel<T>
// // where T: DataModel
// {
//     // let mut x: DataModel<T>;
//     // if let Some(p) = c.parent() {
//          get_parent(p)

//     // x
// }

fn main() {
    let p1 = Test1::default();

    // let mut x = p1;
    if let Some(p) = p1.parent() {
        if let Some(p2) = p.parent() {
            // if let Some(p3) = p2.parent() {

            // }
        }
    }

    // p1.parent().parent().parent()
    
    // let p2 = get_parent(p1);
    // let p2 = get_parent(p1 as DataModel<Test1<Test<Demo>>>);
    // if let Some(p2) = p1.parent() {
    //     println!("p2:: {:?}", p2.parent());
    // }
    println!("p1:: {:?}", p1.parent());

    // let p =  Demo {} ;

    // println!("p::{:?}", p.parent());
}
