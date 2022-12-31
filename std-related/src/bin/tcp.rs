use std::io::Write;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use anyhow::Result;
use custom_utils::logger;
use custom_utils::logger::logger_stdout_debug;
use log::{debug, error};


fn main() {
    logger_stdout_debug();
    let mut tcp = TcpStream::connect(("broker-cn.emqx.io", 1883)).unwrap();
    sleep(Duration::from_secs(30));
    let mut data = [0u8; 100];
    debug!("tcp.write");
    if let Err(e) = tcp.write("s".as_bytes()) {
        error!("{:?}", e);
    }
    debug!("sleep");
    sleep(Duration::from_secs(30));
    debug!("tcp.write");
    if let Err(e) = tcp.write("s".as_bytes()) {
        error!("{:?}", e);
    }
    debug!("sleep");
    sleep(Duration::from_secs(30));
}