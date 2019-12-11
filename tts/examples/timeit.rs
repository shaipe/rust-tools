/// ## Usage
/// ```rust
/// use simple_timer::timeit;
/// 
/// fn hello_world() {
///     println!("hello world");
/// }
/// 
/// fn main() {
///     timeit!("time_1", hello_world());
///     timeit!("time_two",
///         {
///             println!("great weather");
///             println!("i agree");
///         }
///     );
/// }
/// ```
#[macro_export]
macro_rules! timeit {
    ($t: literal, $x:expr) => {
        {
            use std::time::Instant;
            let start = Instant::now();
            let res = $x;
            let end = start.elapsed();
            println!("time({}) : {}.{:03}", $t, end.as_secs(), end.subsec_millis());
            res
        }
    };
}