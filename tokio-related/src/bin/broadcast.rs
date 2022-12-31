use anyhow::Result;
/// 1. 当没有rx时，发送消息会报错。
/// 2. 只要存在rx，就会发送成功（即使rx是刚刚订阅的，即使是之前发送失败过）
#[tokio::main]
async fn main() -> Result<()> {
    let tx = {
        let (tx, _) = tokio::sync::broadcast::channel(1024);
        tx
    };
    assert!(tx.send(()).is_err());
    assert!(tx.send(()).is_err());
    assert!(tx.send(()).is_err());
    let mut rx = tx.subscribe();
    assert!(tx.send(()).is_ok());
    assert!(rx.recv().await.is_ok());
    Ok(())
}