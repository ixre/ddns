use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use ddns::conf;
use ddns::dns;
use ddns::dns::dnspod;
use ddns::dns::ip;
use ddns::dns::NameServer;
use clap::{Arg,App};

const VERSION: &str = "1.0";
const RELEASE_DATE: &str = "2018-12-01";

fn print_licence() {
    println!("\nDDNS v{} (release {})", VERSION, RELEASE_DATE);
}

fn main() {
    print_licence();


    let args = [
        Arg::with_name("conf").short("c")
            .takes_value(true).default_value("./ddns.conf"),
        Arg::with_name("debug").short("d")
            .takes_value(false),
    ];
    let matches = App::new("ddns").args(&args).get_matches();
    let conf = matches.value_of("conf").unwrap();
    let debug = matches.is_present("debug");

    //let (mut ns_list,mut ns_record,mut ns_dyn_type) =
    let cfg = conf::read_conf(conf);
    let mut ns_list = vec![];
    // String is domain name
    let mut ns_record: Vec<HashMap<String, Vec<dns::Record>>> = vec![];
    // Dns record ip map
    let mut ns_dyn_type: Vec<HashMap<String, i8>> = vec![];
    for sp in cfg.dns_config {
        let mut ns = dnspod::DnsPod::new(sp.api_id, sp.api_token);
        let mut domain_map = HashMap::new();
        let mut dyn_type_map = HashMap::new();
        for d in sp.domains {
            let domain = d.domain.clone();
            println!("\nChecking records for {} : ", &domain);
            let mut domain_vec = Vec::new();
            // Join DNS record to map and save dyn-ip type
            let mut j = 1;
            for r in d.records {
                if let Some(record) = ns.get_record_type(&domain, &r.name, dns::RECORD_TYPE_A) {
                    println!("({}) {}.{} | pub:{} | ttl:{}",
                             j, &r.name, &domain, r.dyn_pub, &r.ttl);
                    dyn_type_map.insert(record.id.to_owned(), r.dyn_pub);
                    domain_vec.push(record);
                    j += 1;
                }
            }
            domain_map.insert(domain, domain_vec);
        }
        ns_record.push(domain_map);
        ns_dyn_type.push(dyn_type_map);
        ns_list.push(ns);
    }

    println!("\nDDNS serving ...\n");

    let sp = ip::new(ip::SpNames::ORG3322);
    let in_sp = ip::new(ip::SpNames::Internal);
    loop {
        let addr = sp.addr();
        let in_addr = in_sp.addr();
        let mut i = 0;
        for ns in &mut ns_list {
            let mut map = ns_record.get_mut(i).unwrap();
            for (domain, v) in map {
                for rec in v {
                    let dyn_type = ns_dyn_type.get(i).unwrap().get(&rec.id).unwrap();
                    let ip_addr = if *dyn_type == 1_i8 {
                        addr.to_owned()
                    } else {
                        in_addr.to_owned()
                    };
                    rec.set_value(ip_addr.to_owned());
                    if let Err(err) = ns.update_record(&domain, rec) {
                        println!("[ DDNS][ DNS]: update record failed! {}", err);
                    } else {
                        println!(
                            "[ DDNS][ DNS]: sync dns record :{}=>{}",
                            rec.sub.to_owned(),
                            ip_addr
                        );
                    }
                }
            }
            i += 1;
        }
        thread::sleep(Duration::from_secs(60));
    }
}
