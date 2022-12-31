use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    assert!(tx.send(()).is_ok());
    assert!(rx.await.is_ok());
    Ok(())
}