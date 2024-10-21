#[test]
fn x() {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut my_vec = vec![1, 2, 3, 4, 5, 6];
    my_vec.shuffle(&mut thread_rng());
    println!("{:?}", my_vec);
    println!("{:?}", my_vec.choose(&mut thread_rng()));
    println!("{:?}", my_vec.choose(&mut thread_rng()));
    println!("{:?}", my_vec.choose(&mut thread_rng()));
    println!("{:?}", my_vec.choose(&mut thread_rng()));
    println!("{:?}", my_vec.choose(&mut thread_rng()));
}
