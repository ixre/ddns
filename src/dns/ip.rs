extern crate hostname;
extern crate std;

use hostname::get_hostname;
use std::error::Error;
use std::io;
use std::net;
use std::net::IpAddr;
use std::net::ToSocketAddrs;

// 获取公网IP
pub trait IpAddrFetch {
    fn addr(&self) -> String;
}

// IP SP Names
pub enum SpNames {
    // 内网IP
    Internal,
    // IpIp.net
    IpIpNet,
    // myexternalip.com
    MyIP,
    // 3322
    ORG3322,
}

pub fn new<'a>(s: SpNames) -> &'a IpAddrFetch {
    match s {
        SpNames::Internal => &InternalIp {},
        SpNames::IpIpNet => &IpIp {},
        SpNames::ORG3322 => &ORG3322 {},
        SpNames::MyIP => &MyExternalIP {}
    }
}

struct IpIp {}

impl IpAddrFetch for IpIp {
    fn addr<'a>(&self) -> String {
        unimplemented!()
    }
}


// Get local ip
struct InternalIp {}

impl InternalIp {
    fn ip(&self) -> Result<String, io::Error> { // 这里是std::error::Error
        let host = get_hostname().unwrap();
        let ip_arr: io::Result<Vec<IpAddr>> = (host.as_str(), 0).to_socket_addrs()
            .map(|iter| iter.map(|socket_address| socket_address.ip()).collect());
        if ip_arr.is_ok() {
            let arr = ip_arr?; // 这里是io::Error
            let arr = arr.first().unwrap();
            match arr {
                IpAddr::V4(ip4) => {
                    let mut s = String::from("");
                    for i in &ip4.octets() {
                        if s.len() > 0 {
                            s.push_str(".");
                        }
                        s.push_str(&(*i as u32).to_string());
                    }
                    return Ok(s);
                }
                IpAddr::V6(_ip6) => ()
            }
        }
        return Ok(String::from("127.0.0.1"));
    }
}

impl IpAddrFetch for InternalIp {
    fn addr(&self) -> String {
        return self.ip().unwrap();
    }
}


struct ORG3322 {}

impl IpAddrFetch for ORG3322 {
    fn addr(&self) -> String {
        unimplemented!()
    }
}

struct MyExternalIP {}

impl IpAddrFetch for MyExternalIP {
    fn addr(&self) -> String {
        unimplemented!()
    }
}

