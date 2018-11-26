use std::fs::File;
use std::io;
use std::process::exit;

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsConfig {
    check_seconds: i16,
    pub dns_config: Vec<DnsSP>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsSP {
    pub dns_sp: String,
    pub api_id: String,
    pub api_token: String,
    pub domains: Vec<DnsDomain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsDomain {
    pub domain: String,
    pub records: Vec<DnsRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsRecord {
    // Record name,like "*.dns"
    pub name: String,
    // ttl
    pub ttl: i16,
    // Dynamic public ip, If equals 0 will be update local network ip address.
    pub dyn_pub: i8,
}

pub fn read_conf(path: &str) -> DnsConfig {
    return match File::open(path) {
        Err(ref err) => {
            let conf = default_config();
            if err.kind() == io::ErrorKind::NotFound {
                if let Ok(r) = File::create(path) {
                    let _ = serde_json::to_writer_pretty(r, &conf);
                }
            }
            return conf;
        }
        Ok(r) => {
            let conf = serde_json::from_reader::<File, DnsConfig>(r);
            return match conf {
                Ok(c) => c,
                Err(err) => {
                    println!("[ DDNS][ Config]: Load config failed :{}", err);
                    exit(1);
                }
            };
        }
    };
}

fn default_config() -> DnsConfig {
    let records = vec![DnsRecord {
        name: String::from("@"),
        ttl: 600,
        dyn_pub: 1,
    }];
    let domains = vec![DnsDomain {
        domain: String::from("to2.net"),
        records,
    }];
    let dns_config = vec![DnsSP {
        dns_sp: String::from("dnspod"),
        api_id: String::from(""),
        api_token: String::from(""),
        domains,
    }];
    return DnsConfig {
        check_seconds: 60_i16,
        dns_config,
    };
}

//bak: "73841", "c45f9a093c15daf7c74bfb9bdccace10"
/*
  3322.org
  3322.org (static)
  DynDNS
  no-ip.com
  DHS
  ODS
  DyNS
  HN.ORG
  ZoneEdit
  GNUDip
  easyDNS
  EZ-IP
  TZO
  Meibu
*/
