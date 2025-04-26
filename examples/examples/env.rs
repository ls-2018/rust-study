use std::env;

fn main() {
    let path = std::env::var("IMPORTANT_PATH").unwrap_or("asdas".parse().unwrap());

    println!("{}", path);

    let abs_path = std::env::current_dir().expect("").join("a.txt");
    println!("{:?}", abs_path);

    env::args().for_each(|x| {
        println!("{x}");
    })
}
