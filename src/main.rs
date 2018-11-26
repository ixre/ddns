use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use ddns::conf;
use ddns::dns;
use ddns::dns::dnspod;
use ddns::dns::ip;
use ddns::dns::NameServer;

fn main1() {
    let mux = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _i in 0..5 {
        let mux = mux.clone();
        let h = thread::spawn(move || {
            let mut s = mux.lock().unwrap();
            *s += 1;
        });
        handlers.push(h);
    }
    for h in handlers {
        h.join().unwrap();
    }
    println!("{:#?}", *mux.lock().unwrap())
}

fn main2() {
    let (tx, rx) = mpsc::channel();
    let h = thread::spawn(move || {
        let sp = ip::new(ip::SpNames::Internal);
        let addr = sp.addr();
        tx.send(addr).unwrap();
        thread::sleep(Duration::from_secs(1));
    });
    h.join().unwrap();
    // 单次接收
    //println!("value = {}", &rx.recv().unwrap());
    //println!("value = {}", &rx.recv().unwrap());
    for v in rx { println!("value = {}", v); }
}

fn main() {
    let cfg = conf::read_conf("ddns.conf");
    let mut ns_list = vec![];
    // String is domain name
    let mut ns_record: Vec<HashMap<String, Vec<dns::Record>>> = vec![];
    let mut ns_domains: Vec<Vec<dns::Domain>> = vec![];
    for sp in cfg.dns_config {
        let mut ns = dnspod::DnsPod::new(sp.api_id, sp.api_token);
        let mut domain_map = HashMap::new();
        for d in sp.domains {
            let domain = d.domain.clone();
            let mut domain_vec = Vec::new();
            for r in d.records {
                if let Some(record) = ns.get_record_type(
                    &domain, &r.name, dns::RECORD_TYPE_A) {
                    domain_vec.push(record);
                }
            }
            domain_map.insert(domain, domain_vec);
        }
        ns_record.push(domain_map);
        ns_list.push(ns);
    }

    let sp = ip::new(ip::SpNames::ORG3322);
    let inSp = ip::new(ip::SpNames::Internal);
    loop {
        let addr = sp.addr();
        let mut i = 0;

        for ns in &ns_list {
            for (domain,mut v) in ns_record.get(i).unwrap() {
                for rec in v {
                    //rec.set_value(addr.to_owned());
                    //if let Err(err) = ns.update_record(&domain, rec) {
                    //    println!("[ DDNS][ DNS]: update record failed! {}", err);
                    // }
                }
            }
            i += 1;
        }
        //for (ref mut r) in &mut record_list {
        //    r.set_value(addr.to_owned());
        // if let Err(err) = ns.update_record(domain, r) {
        //     println!("[ DDNS][ DNS]: update record failed! {}", err);
        // }
        //}
        thread::sleep(Duration::from_secs(60));
    }
    //let d = ns.get_domain(domain);
    //println!("{:?}", dm.unwrap());
    //println!("{:?}", record);
    //let callback = move|addr:&str|{ns.update_record(record.unwrap());};
    // dns::sync_ip(ip::SpNames::ORG3322,10,&callback)
    //dns::sync_public_ip(ip::SpNames::ORG3322, 5);//,callback);
    //loop {}

    /*
    loop {
        unsafe {
            if dns::PUBLIC_IP_ADDR.is_some() {
                println!(
                    "[ DDNS][ IP]: internal ip is {:?}",
                    dns::PUBLIC_IP_ADDR.clone().unwrap()
                );
            }
        }
    }*/
}
