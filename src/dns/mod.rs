use std::thread;
use std::time::Duration;

pub mod ip;

// internal network ip address
pub static mut INTERNAL_IP_ADDR: Option<String> = None;
pub static mut PUBLIC_IP_ADDR: Option<String> = None;

pub fn sync_internal_ip(sec:u8) {
    thread::spawn(move || {
        let sp = ip::new(ip::SpNames::Internal);
        loop {
            // 获取本机的IP
            let addr = sp.addr();
            // 保存IP到全局静态变量
            unsafe {
                INTERNAL_IP_ADDR = Some(addr);
            }
            thread::sleep(Duration::from_secs(sec as u64));
        }
    });
}

pub fn sync_public_ip(sp:ip::SpNames,sec:u8){
    thread::spawn(move || {
        let sp = ip::new(sp);
        loop {
            // 获取本机的IP
            let addr = sp.addr();
            // 保存IP到全局静态变量
            unsafe {
                PUBLIC_IP_ADDR = Some(addr);
            }
            thread::sleep(Duration::from_secs(sec as u64));
        }
    });
}