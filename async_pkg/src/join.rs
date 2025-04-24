use futures::executor::block_on;
use futures::{TryFutureExt, join, try_join};

#[derive(Debug)]
struct Book {}
#[derive(Debug)]
struct Music {}

async fn get_book() -> Result<Book, i32> {
    println!("get_book");
    Err(1)
}
async fn get_music() -> Result<Music, String> {
    println!("get_music");
    Ok(Music {})
}

async fn get_book_and_music() -> Result<(Music, Book), i32> {
    let book_fut = get_book();
    let music_fut = get_music().map_err(|_| 1);
    try_join!(music_fut, book_fut) // 遇到Err 会提前结束,依次执行
}

#[test]
fn main() {
    println!("Hello, world! {:?}", block_on(get_book_and_music()));
}
