#![allow(unused)]

enum _IpAddrKind {
    V4,
    V6,
}

// #[allow(dead_code)]
#[derive(Debug)]

pub(crate) struct User {
    active: bool,
    username: String,
    age: u64,
}

#[test]

fn main() {
    // 字节串的类型是字节的数组，而不是字符串了
    let byte_string: &[u8; 21] = b"this is a byte string"; // 数组
    println!("A byte string: {:?}", byte_string);

    // 可以看看下面这串打印出什么
    let escaped = b"\x52\x75\x73\x74 as bytes";

    println!("Some escaped bytes: {:?}", escaped);

    // 字节串与原始字面量结合使用
    let raw_byte_string = br"\u{211D} is not escaped here";

    println!("{:?}", raw_byte_string);

    // ------------------------------------------------------------------------------------------

    let _v: Vec<i32> = Vec::new();

    let _v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);

    v.push(6);

    v.push(7);

    v.push(8);

    let s1 = String::from("superman 1");

    let s2 = String::from("superman 2");

    let s3 = String::from("superman 3");

    let s4 = String::from("superman 4");

    let v = vec![s1, s2, s3, s4, "sad".parse().unwrap()];

    println!("{:?}", v[0]);

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.insert(String::from("Yellow"), 50);

    let x: (i32, f64, u8) = (500, 6.4, 1); // 元组
    // 元组使用.运算符访问其元素，下标从0开始，注意语法
    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        age: 1,
    };

    print!("{}", user1.username);

    let _four = _IpAddrKind::V4;

    let _six = _IpAddrKind::V6;

    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    let x = "6";

    println!("The value of x is: {x}");

    let active = true;

    let username = String::from("someusername123");

    let email = String::from("someone@example.com");

    let user1 = User { active, username, age: 0 };

    let user2 = User {
        username: String::from("another@example.com"),
        ..user1 // 注意这里，直接用 ..user1
    };

    let black = Color(0, 0, 0);

    let module = ArticleModule; // 请注意这一句，也做了实例化操作
    println!("{:?}", user2);

    let mut s1 = String::from("I am a superman.你好啊");

    println!("{s1}");

    foo(&mut s1); // 注意这里传的是字符串的可变引用 &mut s1
    println!("{s1}");

    println!("{}", s1.as_bytes().len());

    for i in s1.as_bytes() {
        //
        println!("{i}")
    }

    let s: &str = "I am a superman.";

    let _ = s.as_bytes();

    let s1: String = String::from(s); // 使用 String 的from构造器
    let s2: String = s.to_string(); // 使用 to_string() 方法;只是转成字符串而已，这两个用法重叠，但是不完全相同。
    let s3: String = s.to_owned(); // 使用 to_owned() 方法
    // 是更通用的，目的就是在只能拿到引用的情况下获得所有权（通过克隆一份资源）。
    //
}

fn foo(s: &mut String) {
    s.push_str(" You are batman.");
}

struct Class {
    serial_number: u32,
    grade_number: u32,
    entry_year: String,
    members: Vec<User>,
}

struct Color(i32, i32, i32); // 元组结构体
struct ArticleModule; // 单元结构体
struct List<T>(Vec<T>);

// 你可以试着编译这段代码
use std::{collections::HashMap, fmt::Debug};

type AAA = HashMap<String, Vec<u8>>;

type BBB = Vec<AAA>;

type CCC = HashMap<String, BBB>;

type DDD = Vec<CCC>;

type EEE = HashMap<String, DDD>;

#[test]

fn _ref() {
    let a = User {
        active: false,
        username: "String".to_string(),
        age: 1,
    };

    let User {
        ref username, // 只是需要读取一下字段的值而已，不需要获得它的所有权,没有再创建一份新实例
        active,
        age,
    } = a;

    println!("{}", active);

    println!("{}", age);

    println!("{}", username);

    println!("{:?}", a);
    println!("{:?}", a.username);
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));

    let a = User {
        active,
        username: "".to_string(),
        age,
    };
    match a {
        obj @ User { .. } => {
            obj // 直接返回,不会像这边发生所有权的转移
        }
        User {
            active: var1,
            username: var2,
            age: 12,
        } => User {
            active: var1,
            username: "x".parse().unwrap(),
            age: 12,
        },
    };
}
