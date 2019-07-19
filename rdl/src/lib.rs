use std::thread;

#[no_mangle]
pub extern fn process(){
    let handles :Vec<_> =(0..100).map(|_|{
        thread::spawn(||{
            let mut x= 0;
            for _ in 0..500000000 {
                x+=1
            }
            x
        })
    }).collect();

    let xc = handles.len();

    for h in handles{
        println!("Thread finished with count={}",h.join().map_err(|_| "Could not join a thread!").unwrap());
    }

    println!("done!{:?}", xc);
}