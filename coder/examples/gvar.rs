use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(vec![]));

    // `tx` is the "transmitter" or "sender".
    // `rx` is the "receiver".
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            // *data += 1;
            data.push("x");
            println!("{}", "y");
            tx.send(()).unwrap();
            
            thread::sleep(Duration::from_millis(50));
        });
    }

    for _ in 0..10 {
        println!("{:?}", rx.recv().unwrap());
    }

    println!("{:?}", data);
}


// use std::thread;
// use std::sync::mpsc;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     for i in 0..10 {
//         let tx = tx.clone();

//         thread::spawn(move || {
//             let answer = i * i;

//             tx.send(answer).unwrap();
//         });
//     }

//     for _ in 0..10 {
//         println!("{}", rx.recv().unwrap());
//     }
// }