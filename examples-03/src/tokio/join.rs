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
    use tokio::runtime::Builder;

    #[test]
    pub fn entry() {
        Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(super::main());
    }
}
