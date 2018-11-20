use std::collections::HashMap;
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

    // Add extra post params
    fn append_params(&self, params: &mut HashMap<&str, String>) {
        params.insert("format", "json".to_owned());
        params.insert("lang", "en".to_owned());
        params.insert("error_on_empty", "on".to_owned());
        params.insert("login_token", self.get_login_token());
        // 仅代理用户需要使用以下参数
        params.insert("user_id", "".to_owned());
    }

    fn build_req(&self, api: &str) -> reqwest::RequestBuilder {
        let mut url = String::from(api_server);
        url.push_str("/");
        url.push_str(api);
        return reqwest::Client::new()
            .post(url.as_str())
            .header("User-Agent", "Rust-ddns/1.0(jarrysix@gmail.com)");
    }


    fn post(&self, api: &str, params: &mut HashMap<&str, String>) -> String {
        self.append_params(params);
        let mut p = vec![];
        for k in params.keys() {
            p.push((*k, params.get(k).unwrap().as_str()));
        }
        let rsp = self.build_req("Domain.List")
            .form(&p)
            .send();

        if let Ok(mut r) = rsp {
            println!("{:?}", r.text().unwrap());
        } else {
            println!("[ DDNS][ DnsPod]: Get domain fail,{}", rsp.err().unwrap());
        }
        return String::from("");
    }
}

impl<'a> NameServer for DnsPod<'a> {
    fn get_domain<'b, 'c>(&self, name: &'b str) -> Domain<'c> where 'c: 'b {
        let mut params = HashMap::new();
        self.post("get_domain", &mut params);
        return Domain { id: "", name: "" };
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
