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

// Domain
#[derive(Debug)]
pub struct Domain {
    id: String,
    name: String,
}

impl Domain {
    pub fn new(id: String, name: String) -> Self {
        return Domain { id, name };
    }
}

// Dns record
pub struct Record<'a> {
    id: &'a str,
    domain_id: &'a str,
    sub: &'a str,
    record_type: i8,
    value: &'a str,
    ttl: i16,
}

impl<'a> Record<'a> {
    fn new(
        id: &'a str,
        domain_id: &'a str,
        sub: &'a str,
        record_type: i8,
        value: &'a str,
        ttl: i16,
    ) -> Record<'a> {
        return Record {
            id,
            domain_id,
            sub,
            record_type,
            value,
            ttl,
        };
    }
}

// Name server like dyn,dnspod
pub trait NameServer {
    /// Get domain by name
    fn get_domain(&mut self, name: &str) -> Option<&Domain>;
    /// Get sub domain, @sub is sub-domain name
    fn get_sub_domain<'a, 'b>(&mut self, sub: &'b str) -> Record<'b>
    where
        'b: 'a;
    /// Update dns record
    fn update_record<T: Error + Sized>(&self, record: Record) -> Result<String, T>;
}
