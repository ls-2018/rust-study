use futures::executor::block_on;
use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, Stream, StreamExt},
};

async fn get_new_num() -> u8 {
    5
}

async fn run_on_new_num(_: u8) {}

async fn run_loop(mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin) {
    let run_on_new_num_fut = run_on_new_num(1).fuse();
    let get_new_num_fut = Fuse::terminated(); // 创建一个空的已经被终止的(terminated)future

    pin_mut!(run_on_new_num_fut, get_new_num_fut);

    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // The timer has elapsed. Start a new `get_new_num_fut`
                // if one was not already running.
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // A new number has arrived -- start a new `run_on_new_num_fut`,
                // dropping the old one.
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            () = run_on_new_num_fut => {},
            // panic if everything completed, since the `interval_timer` should
            // keep yielding values indefinitely.
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

#[test]
fn test_count() {}
