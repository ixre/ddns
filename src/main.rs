use ddns::dns;
use ddns::dns::dnspod;
use ddns::dns::ip;
use ddns::dns::NameServer;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

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
        tx.send(sp.addr()).unwrap();
        // tx.send(1).unwrap();
        thread::sleep(Duration::from_secs(1));
    });
    h.join().unwrap();
    // 单次接收
    println!("value = {}", &rx.recv().unwrap());
    //println!("value = {}", &rx.recv().unwrap());
    //for v in rx { println!("value = {}", v); }
}

fn main() {
    dns::sync_internal_ip(5);
    dns::sync_public_ip(ip::SpNames::ORG3322, 5);

    thread::sleep(Duration::from_secs(5));

    let ns = dnspod::DnsPod::new("73841", "c45f9a093c15daf7c74bfb9bdccace10", "to2.net", "*.dev", 5);
    ns.get_domain("");
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
