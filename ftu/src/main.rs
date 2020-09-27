


fn main() {
    println!("Hello, world!");
    test_p();
}

#[cfg(not(feature = "mysql"))]
fn test_p() {
    println!("sql")
}

#[cfg(feature = "mysql")]
fn test_p(){
    println!("mysql")
}