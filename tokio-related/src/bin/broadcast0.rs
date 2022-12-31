use anyhow::Result;
/// 1. 当容量满时，tx可以继续发送，但会挤掉最开始的消息
/// 2. 当容量满时，rx首次接收会报错，但后续会正常接收消息，不会接收被挤掉的消息
#[tokio::main]
async fn main() -> Result<()> {
        let (tx, mut rx) = tokio::sync::broadcast::channel(2);
    tx.send(1).unwrap();
    tx.send(2).unwrap();
    tx.send(3).unwrap();
    tx.send(4).unwrap();
    assert!(rx.recv().await.is_err());
    while let Ok(data) = rx.recv().await {
        println!("{}", data);
    }
    Ok(())
}