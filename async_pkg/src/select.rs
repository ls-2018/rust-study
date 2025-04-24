use futures::{executor::block_on, future, select};
use std::time::Duration;
use tokio::time;

async fn count() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;
    // default 用在你模式啥都没匹配到,但是已经结束咧~也就是都执行完了。
    // complete 用在当所有分支都完成并且都被选择了的场景。一般select!搭配循环且在结束的时候被调用。
    loop {
        select! {// 使用select!的前提是future得实现Unpin和FusedFuture这俩trait
            b = b_fut => {
                println!("b");
                total += b
            },
            a = a_fut => {
                println!("a");
                total += a
            },
            complete => {
                println!("over");
                break // loop
            },
            default => unreachable!(), // never runs (futures are ready, then complete)
        };
    }
    println!("total {}", total);
}

#[test]
fn main() {
    block_on(count());
}
