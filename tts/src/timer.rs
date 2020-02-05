//! copyright (c) 2020 by shaipe
use std::thread;
use std::time::Duration;

/// 定时间器
pub struct Timer {
    // 时间间隔
    pub interval: u64,

    // 是否运行状态
    running: bool,

    enable: bool,
}

impl Timer {

    pub fn new() -> Self {
        Timer {
            interval: 1000,
            running: false,
        }
    }

    pub fn set_interval(&mut self, interval: u64) {
        self.interval = interval;
    }

    pub fn start(&mut self, elapsed: fn(Timer, u64)) {
        self.execute(elapsed);
    }

    /// 停止
    pub fn stop(&mut self) {

    }

    /// 执行
    fn execute(&mut self, elapsed: fn(Timer, u64)) {
        let mut index = 0u64;
        loop {
            
            elapsed(index);
            index += 1;
            thread::sleep(Duration::from_millis(self.interval));
        }
    }

}