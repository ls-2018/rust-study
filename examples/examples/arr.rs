use ahash::{HashMap, HashMapExt};
use futures::TryStreamExt;
use sqlx::encode::IsNull::No;

fn main() {
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let x: Vec<_> = a.iter().filter(|&&x| x > 3).collect();
    println!("{:?}", x);
}
