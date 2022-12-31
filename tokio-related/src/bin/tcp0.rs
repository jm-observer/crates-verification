use std::io::Write;
use std::time::Duration;
use log::debug;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    custom_utils::logger::logger_stdout_debug();
    let mut tcp = TcpStream::connect(("broker-cn.emqx.io", 1883)).await.unwrap();
    sleep(Duration::from_secs(30)).await;
    let mut data = [0u8; 100];
    debug!("tcp.write");
    if let Err(e) = tcp.try_read(&mut data) {
        println!("{:?}", e);
    }
    if let Err(e) = tcp.write("s".as_bytes()).await {
        println!("{:?}", e);
    }
    debug!("sleep");
    sleep(Duration::from_secs(30)).await;
    debug!("tcp.write");
    if let Err(e) = tcp.write("s".as_bytes()).await {
        println!("{:?}", e);
    }
    debug!("sleep");
    sleep(Duration::from_secs(30)).await;
}