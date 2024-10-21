#[derive(Debug)]
enum MyEnum2<'ax> {
    Add,
    Subtract,
    Mix(&'ax str),
}
#[test]

fn main() {
    let a = MyEnum2::Add;

    match a {
        MyEnum2::Add => println!("a"),
        MyEnum2::Subtract => println!("b"),
        MyEnum2::Mix(val) => println!("val = {val}"),
        // _=>{}
    };

    println!("a = {:?}", a);

    println!("{}", '1' as u8)
}
