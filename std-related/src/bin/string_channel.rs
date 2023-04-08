use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let a = "fewfwe".to_string();
    println!("{:p} {:p} ", &a, a.as_bytes());


    let (tx, rx) = channel();

    thread::spawn(move || {
        let a: String = rx.recv().unwrap();
        println!("{:p} {:p} ", &a, a.as_bytes());
    });
    tx.send(a).unwrap();
    sleep(Duration::from_secs(5));
}