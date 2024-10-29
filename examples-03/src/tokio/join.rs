use std::time::Duration;
use tokio::{join, time::sleep};

async fn task1() {
    sleep(Duration::from_millis(10)).await;
}

async fn task2() {
    sleep(Duration::from_millis(50)).await;
}

async fn main() {
    let t1 = task1();
    let t2 = task2();
    join!(t1, t2);
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn entry() {
        super::main().await
    }
}
