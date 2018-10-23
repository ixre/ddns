extern crate ddns;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let h = thread::spawn(move || {
        loop{
            tx.send(1).unwrap(); // 可写多次
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 单次接收
    // println!("value = {}", rx.recv().unwrap());
    for v in rx{ println!("value = {}",v );}
    h.join().unwrap();
}
