use std::collections::HashMap;
use std::error::Error;

use super::{Domain, NameServer, Record};

const DNS_API_SERVER: &str = "https://dnsapi.cn";

pub struct DnsPod {
    pub api_id: String,
    pub api_token: String,
    pub domains: HashMap<String, Domain>,
}

impl DnsPod {
    pub fn new(api_id: String, api_token: String) -> Self {
        return DnsPod {
            api_id,
            api_token,
            domains: HashMap::new(),
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
                        .insert(d.name.to_owned(),
                                Domain {
                                    id: d.id.to_string(),
                                    name: d.name,
                                    records: vec![],
                                });
                }
            }
            Err(err) => println!("[ DDNS][ Dnspod]: fetch domain list failed :{}", err),
        }
    }

    fn record_type(&self, s: &str) -> i8 {
        return match s {
            "A" => super::RECORD_TYPE_A,
            "CNAME" => super::RECORD_TYPE_CNAME,
            "TXT" => super::RECORD_TYPE_TXT,
            "MX" => super::RECORD_TYPE_TXT,
            _ => 0_i8,
        };
    }

    fn record_type_text(&self, i: i8) -> &str {
        return match i {
            super::RECORD_TYPE_A => "A",
            super::RECORD_TYPE_CNAME => "CNAME",
            super::RECORD_TYPE_TXT => "TXT",
            super::RECORD_TYPE_TXT => "MX",
            _ => "",
        };
    }
}

impl NameServer for DnsPod {
    fn get_domain(&mut self, domain: &str) -> Option<&Domain> {
        self.check_domains();
        if self.domains.contains_key(domain) {
            return self.domains.get(domain);
        }
        return None;
    }

    fn get_record(&mut self, domain: &str, sub: &str) -> Vec<Record> {
        if let Some(d) = self.get_domain(domain) {
            let domain_id = d.id.to_owned();
            let mut params = HashMap::new();
            params.insert("domain_id", domain_id.clone());
            params.insert("keyword", sub.to_owned());
            let rsp = self.post("Record.List", &mut params).replace("\"type\":", "\"type_\":");
            match serde_json::from_str::<RecordListResult>(&rsp) {
                Ok(data) => {
                    let mut arr = vec![];
                    for r in data.records {
                        arr.push(Record {
                            id: r.id.to_owned(),
                            domain_id: domain_id.clone(),
                            sub: r.name.to_owned(),
                            record_type: self.record_type(&r.type_),
                            record_line: r.line.to_owned(),
                            value: r.value.to_owned(),
                            ttl: r.ttl.parse().unwrap(),
                        })
                    }
                    return arr;
                }
                Err(err) => println!("[ DDNS][ Dnspod]: fetch domain list failed :{}", err),
            }
            return vec![];
        }
        return vec![];
    }

    fn get_record_type(&mut self, domain: &str, sub: &str, rt: i8) -> Option<Record> {
        let arr = self.get_record(domain, sub);
        for a in arr {
            if a.sub == sub.to_owned() && a.record_type == rt {
                return Some(a);
            }
        }
        return None;
    }

    fn update_record(&mut self, domain: &str, record: &Record) -> Result<String, String> {
        if let Some(d) = self.get_domain(domain) {
            let domain_id = d.id.to_owned();
            let mut params = HashMap::new();
            params.insert("domain_id", domain_id.clone());
            params.insert("record_id", record.id.to_owned());
            params.insert("sub_domain", record.sub.to_owned());
            params.insert("record_type", self.record_type_text(record.record_type).to_owned());
            params.insert("record_line", record.record_line.to_owned());
            params.insert("value", record.value.to_owned());
            params.insert("ttl", record.ttl.to_string());
            let rsp = self.post("Record.Modify", &mut params).replace("\"type\":", "\"type_\":");
            if let Ok(r) = serde_json::from_str::<DnsResult>(&rsp) {
                if r.status.code == "1" {
                    return Ok(r.status.message);
                }
                return Err(r.status.message);
            }
            return Err(rsp);
        }
        return Err(String::from("no such domain"));
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
struct DnsResult {
    status: Status
}

#[derive(Deserialize, Debug)]
struct Status {
    code: String,
    message: String,
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
    type_: String,
    monitor_status: String,
    remark: String,
    use_aqb: String,
    mx: String,
}
