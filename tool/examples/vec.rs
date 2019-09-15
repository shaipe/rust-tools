fn main() {
    let x: Vec<&str> = vec![
        "md",
        "exe",
        "txt"
    ];
    let ext = "mdw";
    println!("{:?}", x.contains(&ext));
}