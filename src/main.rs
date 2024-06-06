use async_std::task::{sleep, spawn};
use std::time::Duration;

#[async_std::main]
async fn main() {
    let sleepus = spawn(sleepus());
    interruptus().await;

    sleepus.await;
}

fn sleepus() -> impl std::future::Future<Output=()> {
    async {
        for i in 1..=10 {
            println!("Sleepus {}", i);
            sleep(Duration::from_millis(500)).await;
        }
    }
}

async fn interruptus() {
    for i in 1..=5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}
