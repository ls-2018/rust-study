#[test]
fn main() {
    use std::env;
    env::args().for_each(|x| {
        println!("{x}");
    })
}
