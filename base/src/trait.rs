use crate::r#trait::TotalType::B;
use std::fmt::Debug;

#[derive(Debug)]

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    // 注意这一行
    fn play(n: T) {} // 注意这一行
}

impl Point<u32> {
    // 这里，对具化类型 Point<u32> 继续做 impl
    fn doit() {}
}

enum SportType {
    Land,
    Water,
}

trait TraitA {
    type SportType222;

    fn play2(&self, st: Self::SportType222) -> Option<Self::SportType222> {
        Some(st)
    }

    fn play(&self); // 注意这里直接以分号结尾，表示函数签名
    fn play_mut(&mut self);

    fn play_own(self);

    // fn play_some() -> Self { Self }
}

impl TraitA for u8 {
    type SportType222 = SportType;

    // type SportType = SportType::Water;
    fn play2(&self, st: Self::SportType222) -> Option<Self::SportType222> {
        Some(st)
    }

    fn play(&self) {}

    fn play_mut(&mut self) {}

    fn play_own(self) {}
}

impl TraitA for f64 {
    type SportType222 = SportType;

    fn play2(&self, st: Self::SportType222) -> Option<Self::SportType222> {
        Some(st)
    }

    fn play(&self) {}

    fn play_mut(&mut self) {}

    fn play_own(self) {}
}

fn print_trait_a<T: TraitA>(p: Point<T>) {
    <T>::play(&p.x);

    <T as TraitA>::play(&p.x);

    println!("as");
}

struct Foo;

fn main112() {
    let integer = Point::<u8> { x: 5, y: 10 }; // 一个整数point
    let float = Point { x: 1.0, y: 4.0 }; // 一个浮点数point
    // let p = Point { x: Foo, y: Foo };  // 初始化一个Point<T> 实例
    // print(p);
    println!("{:?}", integer);

    println!("{:?}", float);

    print_trait_a(integer);

    print_trait_a(float);

    let both_integer = Point2 { x: 5, y: 10 };

    let both_float = Point2 { x: 1.0, y: 4.0 };

    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // Option
    let x = Some("value");

    println!("{}", x.expect("fruits are healthy"));

    // Result
    // expect:   Option 为None 会 panic
    // expect:   Result 为Err 会 panic

    // Result
    // let path = std::env::var("IMPORTANT_PATH").expect("env variable
    // `IMPORTANT_PATH` should be set by `wrapper_script.sh`"); println!("{}",
    // path);
    let path = std::env::var("IMPORTANT_PATH").unwrap_or("asdas".parse().unwrap());

    println!("{}", path);

    // Option
    let x = Some("air");

    assert_eq!(x.expect("xxxxx"), "air");

    // Result
    let x: Result<u32, &str> = Ok(2);

    assert_eq!(x.unwrap(), 2);

    // Option
    assert_eq!(Some("car").unwrap_or("bike"), "car");

    assert_eq!(None.unwrap_or("bike"), "bike");

    // Result
    let default = 2;

    let n: u32 = 9;

    let x: Result<u32, &str> = Ok(n);

    assert_eq!(x.unwrap_or(default), 9);

    let x: Result<u32, &str> = Err("error");

    assert_eq!(x.unwrap_or(default), default);

    // Option
    let x: Option<u32> = None;

    let y: Option<u32> = Some(12);

    assert_eq!(x.unwrap_or_default(), 0);

    assert_eq!(y.unwrap_or_default(), 12);

    // Result
    let good_year_from_input = "1909";

    let bad_year_from_input = "190blarg";

    let year_from_input = "blarg";

    let good_year: u32 = good_year_from_input.parse().unwrap_or_default();

    let bad_year: u32 = bad_year_from_input.parse().unwrap_or_default();

    let y_year = year_from_input.parse::<String>().unwrap_or_default();

    assert_eq!(1909, good_year);

    assert_eq!(0, bad_year);

    assert_eq!("blarg", y_year);

    let maybe_some_string = Some(String::from("Hello, World!"));

    let maybe_some_len = maybe_some_string.map(|s| -> usize { s.len() }); // 这是一个闭包
    assert_eq!(maybe_some_len, Some(13));

    let x: Option<&str> = None;

    assert_eq!(x.map(|s| s.len()), None);

    let x = 12;

    let opt_x = Some(&x);

    assert_eq!(opt_x, Some(&12));

    let cloned = opt_x.cloned();

    assert_eq!(cloned, Some(12));

    let x: Option<u32> = Some(2);

    assert!(x.is_some());
    let x: Option<u32> = None;

    assert!(!x.is_some());

    let x: Option<u32> = Some(2);

    assert!(!x.is_none());

    let x: Option<u32> = None;

    assert!(x.is_none());

    // 把 Option 或 &Option 转换成 Option<&T>。创建一个新
    // Option，里面的类型是原来类型的引用，就是从 Option 到 Option<&T>。
    // 原来那个 Option 实例保持不变。
    let text: Option<String> = Some("Hello, world!".to_string());

    // let text_length: Option<usize> = text.map(|s| s.len());
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());

    println!("still can print text: {text:?}");

    println!("still can print text: {text_length:?}");

    // 创建一个新 Result，里面的两种类型分别是原来两种类型的引用，就是从 Result 到
    // Result<&T, &E>。原来那个 Result 实例保持不变
    let x: Result<u32, &str> = Ok(2);

    assert_eq!(x.as_ref(), Ok(&2));

    let x: Result<u32, &str> = Err("Error");

    assert_eq!(x.as_ref(), Err(&"Error"));

    // 把 Option 或 &mut Option 转换成 Option<&mut T>。
    let mut x = Some(2);

    // match x.as_mut() {
    //     Some(v) => *v = 42,
    //     None => {}
    // }
    if let Some(v) = x.as_mut() { *v = 42 }

    assert_eq!(x, Some(42));

    // 把 Option 的值拿出去，在原地留下一个 None
    // 值。这个非常有用。相当于把值拿出来用，但是却没有消解原来那个 Option。
    let mut x = Some(2);

    let y = x.take();

    assert_eq!(x, None);

    assert_eq!(y, Some(2));

    let mut x: Option<u32> = None;

    let y = x.take();

    assert_eq!(x, None);

    assert_eq!(y, None);

    // 在原地替换新值，同时把原来那个值抛出来。
    let mut x = Some(2);

    let old = x.replace(5);

    assert_eq!(x, Some(5));

    assert_eq!(old, Some(2));

    let mut x = None;

    let old = x.replace(3);

    assert_eq!(x, Some(3));

    assert_eq!(old, None);

    // and_then 如果 Option 是 None，返回 None；如果 Option 是
    // Some，就把参数里面提供的函数或闭包应用到被包裹的内容上，并返回运算后的结果。
    fn sq_then_to_string(x: u32) -> Option<String> {
        x.checked_mul(x).map(|sq| sq.to_string()) // 相乘
    }

    assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));

    assert_eq!(Some(3).and_then(sq_then_to_string), Some(9.to_string()));

    assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
    assert_eq!(None.and_then(sq_then_to_string), None);

    let x: u32 = 10_000;

    println!("{:?}", x);

    println!("{:?}", x.checked_mul(x));

    let line = "1\n2\n3\n4\n";

    for num in line.lines() {
        // match num.parse::<i32>().map(|i| i * 2) {
        //     Ok(n) => println!("{n}"),
        //     Err(..) => {}
        // }
        // 等价
        if let Ok(n) = num.parse::<i32>().map(|i| i * 2) { println!("{n}") }
    }

    let x: Result<i32, &str> = Ok(-3);

    assert!(x.is_ok());

    let x: Result<i32, &str> = Err("Some error message");

    assert!(!x.is_ok());

    let x: Result<i32, &str> = Ok(-3);

    assert!(!x.is_err());

    let x: Result<i32, &str> = Err("Some error message");

    assert!(x.is_err());

    fn stringify(x: u32) -> String {
        format!("error code: {x}")
    }

    let x: Result<u32, u32> = Ok(2);

    assert_eq!(x.map_err(stringify), Ok(2));

    let x: Result<u32, u32> = Err(13);

    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));

    let x = Some("foo"); // Option->Result
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: Option<&str> = None;

    assert_eq!(x.ok_or(0), Err(0));

    let x: Result<u32, &str> = Ok(2); // Result->Option
    assert_eq!(x.ok(), Some(2));

    let x: Result<u32, &str> = Err("Nothing here");

    assert_eq!(x.ok(), None);

    let x: Result<u32, &str> = Ok(2);

    assert_eq!(x.err(), None);

    let x: Result<u32, &str> = Err("Nothing here");

    assert_eq!(x.err(), Some("Nothing here"));
}

trait TraitA2 {
    const LEN: u32 = 10;

    type MYType: Debug;
}

trait TraitA3: TraitA2 {
    // 继承
    const LEN: u32 = 123;
}

trait TraitA4
where
    Self: TraitA2,
{
    // 继承
    const LEN: u32 = 123;
}

fn doit<T: TraitA2>(a: T::MYType) {} // 这里在函数中使用了关联类型

struct TypeA;

impl TraitA2 for TypeA {
    type MYType = String;

    const LEN: u32 = 123; // 具化关联类型为String
}

struct Foo2<T: TraitA2<MYType=String>> {
    // 这里在约束表达式中对关联类型做了具化
    x: T,
}

struct Aaaa;

impl TraitA2 for Aaaa {
    const LEN: u32 = 120;

    type MYType = String;
}

#[test]

fn main() {
    doit::<TypeA>("abc".to_string()); // 给Rustc小助手喂信息：T具化为TypeA
    let a = Foo2 { x: Aaaa };

    println!("{:?}", Aaaa::LEN);

    println!("{:?}", <TypeA as TraitA2>::LEN); // 这种语法，叫做完全限定语法
}

fn doit22<T>()
where
    T: TraitA,
    T::SportType222: Debug + PartialEq,
{
    // 使用where语句将T的约束表达放在后面来
    // 注意这一句，直接对TraitA的关联类型Item添加了更多一个约束 PartialEq
}

trait TraitA22 {}

trait TraitB22 {}

impl<T: TraitB22> TraitA22 for T {} // 这里直接对T进行实现TraitA

trait TraitA33<T> {
    fn p(t: T);
}

struct Atype;

impl<T> TraitA33<T> for Atype {
    fn p(t: T) {}
}

// impl TraitA33<u8> for Atype {
//     fn p(t: u8) {
//     }
// }
// impl TraitA33<u32> for Atype {}

trait TraitA55<T> {}

struct Atype55<U> {
    a: U,
}

impl<T, U> TraitA55<T> for Atype55<U>
where
    T: Debug,     // 在 impl 时添加了约束
    U: PartialEq, // 在 impl 时添加了约束
{}

struct Btype;

struct Ctype;

enum TotalType {
    B(Btype),
    C(Ctype),
}

fn doit3(i: u32) -> TotalType {
    // 返回枚举类型
    if i == 0 {
        let b = Btype;

        B(b)
        // TotalType::B(b)    // 在这个分支中返回变体B
    } else {
        let c = Ctype;

        TotalType::C(c) // 在这个分支中返回变体C
    }
}

fn foo2(a: &u32, b: &u32) {
    if a > b {
        println!("111");
    } else {
        println!("222");
    }
}

#[derive(Debug)]

enum MyEnum2<'ax> {
    Add,
    Subtract,
    Mix(&'ax str),
}
fn xxx() {
    trait TraitA {
        fn foo(&self);
    }

    trait TraitB {
        fn bar(&self);
    }

    impl<T: TraitB> TraitA for T {
        fn foo(&self) {}
    }

    impl TraitA for u32 {
        fn foo(&self) {}
    }

    let x: u32 = 99;

    x.foo();
}

trait Vegetable {}

struct Salad<V: Vegetable> {
    veggies: Vec<V>, // 泛型
}

struct City {}

// fn(&City) -> bool    // fn类型（只接受函数）
// Fn(&City) -> bool    // Fn特型（既接受函数也接受闭包）
struct Salad3 {
    veggies: Vec<Box<dyn Vegetable>>, // 特型对象,Box 存放两个指针
}
fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
where
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}
