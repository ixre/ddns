use std::error::Error;
use super::{Domain, NameServer, Record};

const api_server: &str = "https://dnsapi.cn";


pub struct DnsPod<'a> {
    api_id: &'a str,
    api_token: &'a str,
    domain: &'a str,
    sub_domains: &'a str,
    check_second: i16,
}

impl<'a> DnsPod<'a> {
    pub fn new(api_id: &'a str, api_token: &'a str, domain: &'a str,
               sub_domains: &'a str, check_second: i16) -> Self {
        return DnsPod { api_id, api_token, domain, sub_domains, check_second };
    }
    fn get_login_token(&self) -> String {
        return format!("{0},{1}", self.api_id, self.api_token);
    }
}

impl<'a> NameServer for DnsPod<'a> {
    fn get_domain<'b, 'c>(&self, name: &'b str) -> Domain<'c> where 'c: 'b {
        unimplemented!()
    }

    fn get_sub_domain<'b, 'c>(&self, sub: &'b str) -> Record<'c> where 'c: 'b {
        unimplemented!()
    }

    fn update_record<T: Error + Sized>(&self, record: Record) -> Result<String, T> {
        unimplemented!()
    }

    /*
    fn get_sub_domain(&self, sub: &'b str) -> Record<'a> {
        unimplemented!()
    }

    fn update_record<T: Error + Sized>(&self, record: Record<'a>) -> Result<String, T> {
        unimplemented!()
    }*/
}
