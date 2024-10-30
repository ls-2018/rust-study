#![allow(unused)]
use std::time::Duration;
use tokio::{join, time::sleep};

async fn task1() {
    sleep(Duration::from_millis(10)).await;
}

async fn task2() {
    sleep(Duration::from_millis(50)).await;
}

#[cfg(test)]
mod test {
    use crate::tokio::join::{task1, task2};
    use tokio::join;

    #[tokio::test]
    async fn entry() {
        let t1 = task1();
        let t2 = task2();
        join!(t1, t2);
    }
}
