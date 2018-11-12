extern crate ddns;
extern crate std;

use ddns::dns;
use ddns::dns::ip;
use std::string::String;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::time;
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
    dns::sync_internal_ip();
    loop {
        unsafe {
            if dns::INTERNAL_IP_ADDR.is_some() {
                println!("[ DDNS][ IP]: internal ip is {:?}",
                         dns::INTERNAL_IP_ADDR.clone().unwrap());
            }
        }
    }
}
