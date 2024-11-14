use ahash::{HashMap, RandomState};
fn main() {
    let hash_builder = RandomState::with_seed(42);
    let hash = hash_builder.hash_one("Some Data");
    println!("{}", hash);
    //  v 只能是 数字
    let mut m = HashMap::with_hasher(RandomState::with_seed(42));
    m.insert("key1", 1);
    println!("{:?}", m);
}
