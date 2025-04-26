use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut items = vec![1, 2, 3, 4, 5];

    // 获取一个随机元素
    let mut rng = thread_rng();
    if let Some(random_item) = items.choose(&mut rng) {
        println!("随机选中的元素是: {}", random_item);
    }

    // 打乱整个数组
    items.shuffle(&mut rng);
    println!("打乱后的数组: {:?}", items);
}
