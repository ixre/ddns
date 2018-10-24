extern crate ddns;

use ddns::net::ip;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let h = thread::spawn(move || {
        let sp = ip::new(ip::SpNames::Internal);
        loop {
            tx.send(sp.addr()).unwrap();
            // tx.send(1).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 单次接收
    // println!("value = {}", rx.recv().unwrap());
    for v in rx { println!("value = {}", v); }
    h.join().unwrap();
}
