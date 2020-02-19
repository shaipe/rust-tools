//! copyright (c) 2020 by shaipe
//! 

mod timer;
use timer::Timer;

fn main() {
    
    let mut t = Timer::new();
    t.start(|x, interval|{
        println!("{:?} -- {:?}", x, interval);
    });
    
}