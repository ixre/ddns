use std::thread;
use std::time::Duration;

pub mod ip;

// internal network ip address
pub static mut INTERNAL_IP_ADDR: Option<String> = None;


pub fn sync_internal_ip() {
    thread::spawn(move || {
        loop {
            // 获取本机的IP
            let sp = ip::new(ip::SpNames::Internal);
            let addr = sp.addr();
            // 保存IP到全局静态变量
            unsafe {
                INTERNAL_IP_ADDR = Some(addr);
            }
            thread::sleep(Duration::from_secs(2));
        }
    });
}