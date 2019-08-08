extern crate timer;
extern crate chrono;

use timer::Timer;
use chrono::Duration;
use std::thread;

fn x() {
    println!("hello");
}

fn main() {
    let timer = Timer::new();
    let guard = timer.schedule_repeating(Duration::seconds(2), x);
    // give some time so we can see hello printed
    // you can execute any code here
    thread::sleep(::std::time::Duration::new(10, 0));
    // stop repeating
    drop(guard);
}