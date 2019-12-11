use chrono::offset::Local;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

fn main() {
    let mut app = if let Some(count) = get_count_minutes() {
        App::new(count)
    } else {
        return;
    };
    log("Timer started!");
    loop {
        if app.current_count == 0 {
            log("Time up!");
            break;
        }
        log(&format!(
            "{} minutes remaining.",
            app.current_count_formatted()
        ));
        app.current_count -= 1;
        thread::sleep(Duration::from_secs(60));
    }
}

fn get_count_minutes() -> Option<u32> {
    let args: Vec<String> = std::env::args().collect();
    let expect_num = if let Some(expect_num) = args.get(1) {
        expect_num
    } else {
        eprintln!("Please speficy the time to count in minutes!");
        return None;
    };
    if let Ok(num) = u32::from_str(expect_num) {
        Some(num)
    } else {
        eprintln!("'{}' is not a valid number!", expect_num);
        None
    }
}

fn log(message: &str) {
    let time_str = {
        let time = Local::now();
        time.format("%H:%M").to_string()
    };
    println!("[{}] {}", time_str, message);
}

struct App {
    initial_count: u32,
    current_count: u32,
}

impl App {
    fn new(initial_count: u32) -> Self {
        App {
            current_count: initial_count,
            initial_count,
        }
    }

    fn current_count_formatted(&self) -> String {
        let not_enough_zero = {
            let initial_digit = self.initial_count.to_string().len();
            let current_digit = self.current_count.to_string().len();
            initial_digit - current_digit
        };
        let mut buffer = String::with_capacity(not_enough_zero);
        for _x in 0..not_enough_zero {
            buffer.push('0');
        }
        buffer.push_str(&self.current_count.to_string());
        buffer
    }
}
