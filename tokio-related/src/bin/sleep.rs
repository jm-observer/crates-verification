use std::time::Duration;
use anyhow::Result;
use log::debug;
use tokio::select;
use tokio::time::{Instant, sleep, sleep_until};

/// 1. 当没有rx时，发送消息会报错。
/// 2. 只要存在rx，就会发送成功（即使rx是刚刚订阅的，即使是之前发送失败过）
#[tokio::main]
async fn main() -> Result<()> {
    let a = A::init(Duration::from_secs(3));
    custom_utils::logger::logger_stdout_debug();
    debug!("start");
    let mut time = 5;
    loop {
        select! {
            _ = sleep(Duration::from_secs(2)) =>{
                debug!("sleep");
            }
            _ = a.sleep() =>{
                debug!("a.sleep()");
                time -= 1;
                if time <= 0 {
                    break;
                }
            }
        }
    }
    debug!("end");
    Ok(())
}



struct A {
    deadline: Instant,
}

impl A {
    fn init(after: Duration) -> Self {
        let deadline = Instant::now() + after;
        Self { deadline }
    }
    async fn sleep(&self) {
        sleep_until(self.deadline).await
    }
}