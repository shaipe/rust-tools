use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let service_name = if args.len() > 1 {
        &args[1]
    }
    else{
        "xservice"
    };
    let service_binary_path = if args.len() > 2 {
        &args[2]
    }
    else {
        "xservice.exe"
    };

    let service_descript = if args.len() > 3 {
        &args[3]
    }
    else {
        "service description"
    };

    println!("service name {}, service path: {}, service description: {}", service_name, service_binary_path, service_descript);
}