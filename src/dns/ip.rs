use hostname::get_hostname;
use regex::Regex;
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
        SpNames::IpIpNet => &IpIpNet {},
        SpNames::ORG3322 => &ORG3322 {},
        SpNames::MyIP => &MyExternalIP {}
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


struct IpIpNet {}

impl IpAddrFetch for IpIpNet {
    fn addr<'a>(&self) -> String {
        let req = reqwest::get("https://www.ipip.net/");
        if req.is_ok() {
            let body = req.unwrap().text().unwrap();
            let re = Regex::new("<span>IP[^>]*><a[^>]+>(.+?)</a>").unwrap();
            let cap = re.captures(&body);
            if cap.is_some() {
                return cap.unwrap().get(1).unwrap().as_str().to_owned();
            }
        }
        return String::from("");
    }
}


struct ORG3322 {}

impl IpAddrFetch for ORG3322 {
    fn addr(&self) -> String {
        let req = reqwest::get("http://members.3322.org/dyndns/getip");
        if req.is_ok() {
            let body = req.unwrap().text().unwrap();
            return String::from_utf8(body.as_bytes()[0..body.len()-1].to_vec()).unwrap()
        }
        return String::from("");
    }
}

struct MyExternalIP {}

impl IpAddrFetch for MyExternalIP {
    fn addr(&self) -> String {
        let req = reqwest::get("http://myexternalip.com/raw");
        if req.is_ok() {
           return req.unwrap().text().unwrap();
        }
        return String::from("");
    }
}

