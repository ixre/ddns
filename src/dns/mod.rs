use std::error::Error;
use std::thread;
use std::time::Duration;

pub mod dnspod;
pub mod dyn_;
pub mod ip;

// internal network ip address
pub static mut INTERNAL_IP_ADDR: Option<String> = None;
pub static mut PUBLIC_IP_ADDR: Option<String> = None;

pub fn sync_internal_ip(sec: u8) {
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

/*
pub fn sync_ip<I:ip::IpAddrFetch, T>(sp: I, sec: u8, callback: T)
    where T: Fn(&str) {
    callback(&sp.addr());
}*/

pub fn sync_public_ip(sp: ip::SpNames, sec: u8) {
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

// A Record
pub const RECORD_TYPE_A: i8 = 1;
// CName Record
pub const RECORD_TYPE_CNAME: i8 = 2;
// Txt Record
pub const RECORD_TYPE_TXT: i8 = 3;
// MX Record
pub const RECORD_TYPE_MX: i8 = 4;

// Domain
#[derive(Debug)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub records: Vec<Record>,
}

// Dns record
#[derive(Debug)]
pub struct Record {
    pub id: String,
    pub domain_id: String,
    pub sub: String,
    pub record_type: i8,
    pub record_line: String,
    pub value: String,
    pub ttl: i32,
}

impl Record {
    pub fn set_value(&mut self, s: String) {
        self.value = s;
    }
}

// Name server like dyn,dnspod
pub trait NameServer {
    /// Get domain by name
    fn get_domain(&mut self, domain: &str) -> Option<&Domain>;
    /// Get sub domain, @sub is sub-domain name
    fn get_record(&mut self, domain: &str, sub: &str) -> Vec<Record>;
    // Get domain record and match record types.
    fn get_record_type(&mut self, domain: &str, sub: &str, rt: i8) -> Option<Record>;
    /// Update dns record
    fn update_record(&mut self, domain: &str, record: &Record) -> Result<String, String>;
}
