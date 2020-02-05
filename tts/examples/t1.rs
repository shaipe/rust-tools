use tokio::time::delay_for;

use std::time::Duration;


#[tokio::main]
async fn main() {
    println!("start");
    delay_for(Duration::from_millis(1000)).await;
    println!("100 ms have elapsed");
}