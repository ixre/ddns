use std::net;
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

pub fn new<'a>(s: SpNames) -> &'a IpAddrFetch{
    match s {
        SpNames::Internal => &InternalIp {},
        SpNames::IpIpNet => &IpIp {},
        SpNames::ORG3322 =>&ORG3322 {},
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

        unimplemented!()
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

