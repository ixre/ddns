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
    //dns::sync_internal_ip(5);
    //thread::sleep(Duration::from_secs(2));

    let cfg = conf::read_conf("ddns.conf");
//    let mut ns_list = vec![];
//    let mut record_list: Vec<&mut dns::Record> = vec![];
//    for sp in cfg.dns_config {
//        let mut ns = dnspod::DnsPod::new(sp.api_id, sp.api_token);
//        for d in sp.domains {
//            let record_name = d.domain.clone();
//            for r in d.records {
//                let mut record = ns.get_record_type(
//                    &record_name,
//                    &r.name,
//                    dns::RECORD_TYPE_A).unwrap();
//                //record_list.put(&mut record);
//            }
//        }
//        ns_list.push(&mut ns);
//    }
    let mut ns = dnspod::DnsPod::new("73841".to_owned(), "c45f9a093c15daf7c74bfb9bdccace10".to_owned());
    let domain = "to2.net";
    let sub_domain = "*.dev";
    let mut record = ns.get_record_type(domain, sub_domain, dns::RECORD_TYPE_A);
    if let Some(ref mut r) = record {
        let sp = ip::new(ip::SpNames::ORG3322);
        loop {
            let addr = sp.addr();
            r.set_value(addr);
            if let Err(err) = ns.update_record(domain, r) {
                println!("[ DDNS][ DNS]: update record failed! {}", err);
            }
            thread::sleep(Duration::from_secs(60));
        }
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
