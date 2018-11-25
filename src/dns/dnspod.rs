use std::collections::HashMap;
use std::error::Error;

use super::{Domain, NameServer, Record};

const DNS_API_SERVER: &str = "https://dnsapi.cn";

pub struct DnsPod<'a> {
    api_id: &'a str,
    api_token: &'a str,
    domains: HashMap<String, Domain>,
    check_second: i16,
}

impl<'a> DnsPod<'a> {
    pub fn new(api_id: &'a str, api_token: &'a str, check_second: i16) -> Self {
        return DnsPod {
            api_id,
            api_token,
            domains: HashMap::new(),
            check_second,
        };
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
        let mut url = String::from(DNS_API_SERVER);
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
        let rsp = self.build_req(api).form(&p).send();
        if let Ok(mut r) = rsp {
            return r.text().unwrap();
        } else {
            println!("[ DDNS][ DnsPod]: Get domain fail,{}", rsp.err().unwrap());
        }
        return String::from("");
    }

    fn push_domain(&mut self, d: DomainDeserialize) {}

    /// Check domains and push to domain map.
    fn check_domains(&mut self) {
        if self.domains.len() > 0 {
            return;
        }
        let mut params = HashMap::new();
        let rsp = self.post("Domain.List", &mut params);
        match serde_json::from_str::<DomainListResult>(&rsp) {
            Ok(arr) => {
                for d in arr.domains {
                    self.domains
                        .insert(d.name.to_owned(), Domain::new(d.id.to_string(), d.name));
                }
            }
            Err(err) => println!("[ DDNS][ Dnspod]: fetch domain list failed :{}", err),
        }
    }
}

impl<'a> NameServer for DnsPod<'a> {
    fn get_domain(&mut self, domain: &str) -> Option<&Domain> {
        self.check_domains();
        if self.domains.contains_key(domain) {
            return self.domains.get(domain);
        }
        return None;
    }

    fn get_record<'b, 'c>(&mut self, domain: &str, sub: &'b str) -> Vec<Record<'c>>
        where 'c: 'b, {
        if let Some(d) = self.get_domain(domain) {
            let mut params = HashMap::new();
            params.insert("domain_id", d.id.to_owned());
            params.insert("keyword", sub.to_owned());
            let rsp = self.post("Record.List", &mut params).replace("\"type\":", "\"_type\":");
            match serde_json::from_str::<RecordListResult>(&rsp) {
                Ok(arr) => {
                    println!("{:#?}", arr);
                    //for d in arr.records {
                    //    self.domains
                    //        .insert(d.name.to_owned(), Domain::new(d.id.to_string(), d.name));
                    // }
                }
                Err(err) => println!("[ DDNS][ Dnspod]: fetch domain list failed :{}", err),
            }
            return vec![];
        }
        return vec![];


        /*
        match serde_json::from_str::<DomainListResult>(&rsp) {
            Ok(arr) => {
                for d in arr.domains {
                    self.domains
                        .insert(d.name.to_owned(), Domain::new(d.id.to_string(), d.name));
                }
            }
            Err(err) => println!("[ DDNS][ Dnspod]: fetch domain list failed :{}", err),
        }*/
    }

    fn get_record_type<'b, 'c>(&mut self, domain: &str, sub: &'b str, rt: i8) -> Option<Record<'c>> where 'c: 'b {
        let arr = self.get_record(domain, sub);
        return None;
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

#[derive(Deserialize, Debug)]
struct DomainListResult {
    domains: Vec<DomainDeserialize>,
}

#[derive(Deserialize, Debug)]
struct DomainDeserialize {
    id: i32,
    status: String,
    grade: String,
    group_id: String,
    searchengine_push: String,
    is_mark: String,
    ttl: String,
    cname_speedup: String,
    remark: String,
    created_on: String,
    updated_on: String,
    punycode: String,
    ext_status: String,
    src_flag: String,
    name: String,
    grade_title: String,
    is_vip: String,
    owner: String,
    records: String,
}

#[derive(Deserialize, Debug)]
struct RecordListResult {
    records: Vec<RecordDeserialize>,
}

#[derive(Deserialize, Debug)]
struct RecordDeserialize {
    id: String,
    ttl: String,
    value: String,
    enabled: String,
    status: String,
    updated_on: String,
    name: String,
    line: String,
    line_id: String,
    _type: String,
    monitor_status: String,
    remark: String,
    use_aqb: String,
    mx: String,
}
