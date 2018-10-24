extern crate hostname;
extern crate std;

use hostname::get_hostname;
use std::net;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::io;

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

struct InternalIp {}

impl IpAddrFetch for InternalIp {
    fn addr(&self) -> String {
        let host = get_hostname().unwrap();
        let ip_arr :io::Result<Vec<IpAddr>>= (host.as_str(), 0).to_socket_addrs()
            .map(|iter| iter.map(|socket_address| socket_address.ip()).collect());
        if ip_arr.is_ok(){
           let arr =  ip_arr.unwrap();
            let arr = arr.first().unwrap();
            match arr{
                IpAddr::V4(ip4)=>{
                    let ip = ip4.octets();
                },
                IpAddr::V6(ip6)=>()
            }
           // println!("{:#?}",arr);
        }
        return String::from("");
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

