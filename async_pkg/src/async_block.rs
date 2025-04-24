use futures::future::{BoxFuture, FutureExt};
use futures::{Future, executor::block_on};
use std::pin::Pin;

async fn foo() -> Result<u8, ()> {
    Ok(1)
}

async fn bar() -> Result<u8, ()> {
    Ok(2)
}

fn recursive() -> BoxFuture<'static, ()> {
    async move {
        recursive().await;
        recursive().await;
    }
    .boxed()
}

#[test]
fn main() {
    // let fut: Box<dyn Future<Output = Result<u8, ()>>> = Box::new(async {
    let fut: Pin<Box<dyn Future<Output = Result<u8, ()>>>> = Box::pin(async {
        foo().await?;
        bar().await?;
        Ok(1)
    });
    let res = block_on(fut);

    dbg!(res).expect("TODO: panic message");
}
