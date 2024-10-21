#[derive(Debug)]

struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    // 实现从(i32, i32)到Point的转换
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

impl From<[i32; 2]> for Point {
    // 实现从[i32; 2]到Point的转换
    fn from([x, y]: [i32; 2]) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]

struct Point2 {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point2 {
    // 实现从(i32, i32)到Point的转换
    fn from((x, y): (i32, i32)) -> Self {
        Point2 { x, y }
    }
}

impl From<[i32; 2]> for Point2 {
    // 实现从[i32; 2]到Point的转换
    fn from([x, y]: [i32; 2]) -> Self {
        Point2 { x, y }
    }
}
#[test]
fn example() {
    // 使用from()转换不同类型
    let origin = Point::from((0, 0));

    println!("{:?}", origin);

    let origin = Point::from([0, 1]);

    println!("{:?}", origin);

    // 使用into()转换不同类型
    let origin: Point = (0, 2).into();

    println!("{:?}", origin);

    let origin: Point = [0, 0].into();

    println!("{:?}", origin);

    let origin: Point2 = (0, 2).into();

    println!("{:?}", origin);
}

struct Person {
    name: String,
}
impl Person {
    // 这个方法只接收String参数
    fn new1(name: String) -> Person {
        Person { name }
    }
    // 这个方法可接收
    // - String
    // - &String
    // - &str
    // - Box<str>
    // - char
    // 这几种参数，因为它们都实现了Into<String>
    fn new2<N: Into<String>>(name: N) -> Person {
        Person { name: name.into() } // 调用into()，写起来很简洁
    }
}

use std::str::FromStr;
fn example2<T: FromStr>(s: &str) {
    // 下面4种表达等价
    let t: Result<T, _> = FromStr::from_str(s);
    let t = T::from_str(s);
    let t: Result<T, _> = s.parse();
    let t = s.parse::<T>(); // 最常用的写法
}
