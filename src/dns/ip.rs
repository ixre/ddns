use std::io;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::process::exit;

use regex::Regex;

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
        SpNames::MyIP => &MyExternalIP {},
    }
}

// Get local ip
struct InternalIp {}

impl InternalIp {
    fn ip(&self) -> Result<String, io::Error> {
        // 这里是std::error::Error
        let host = sys_info::hostname().unwrap();
        if host == "localhost" {
            println!("Must update a new name to continue. You can use follow command:\n");
            println!("'echo host > /etc/hostname'; (Permanent) and reboot or \n'sudo hostname host', replace host with your hostname.\n");
            exit(1);
        }
        //println!("{}",&hostx);

        let ip_arr: io::Result<Vec<IpAddr>> = (host.as_str(), 0)
            .to_socket_addrs()
            .map(|iter| iter.map(|socket_address| socket_address.ip()).collect());
        if let Ok(ip_arr) = ip_arr {
            for arr in ip_arr {
                match arr {
                    IpAddr::V4(ip4) => {
                        let mut s = String::from("");
                        for i in &ip4.octets() {
                            if s.len() > 0 {
                                s.push_str(".");
                            }
                            s.push_str(&(*i as u32).to_string());
                        }
                        // Checking ip are local area ip
                        if !s.starts_with("127.") {
                            return Ok(s);
                        }
                    }
                    IpAddr::V6(_ip6) => (),
                }
            }
        }

        /*
        let arr = ip_arr.unwrap().first().unwrap();
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
            IpAddr::V6(_ip6) => (),
        }*/
        return Ok(String::from(""));
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
            return String::from_utf8(body.as_bytes()[0..body.len() - 1].to_vec()).unwrap();
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
