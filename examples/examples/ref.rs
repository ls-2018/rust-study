#![allow(unused)]

// 使用 &str 作为参数可以接收下面两种类型
//  - &str
//  - &String
fn takes_str(s: &str) {
    // use &str
}
// 使用 AsRef<str> 作为参数可以接受下面三种类型
//  - &str
//  - &String
//  - String
fn takes_asref_str<S: AsRef<str>>(s: S) {
    // 它把自身的引用转换成目标类型的引用
    let s: &str = s.as_ref();
    // use &str
}

fn example(slice: &str, borrow: &String, owned: String) {
    let a: &str = borrow.as_ref();
    takes_str(slice);
    takes_str(borrow);
    // takes_str(owned); // ❌
    takes_asref_str(slice);
    takes_asref_str(borrow);
    takes_asref_str(owned); // ✅
}
//  Deref 看成是隐式化（或自动化）+ 弱化版本的 AsRef。
pub(crate) struct User {
    active: bool,
    username: String,
    age: u64,
}

fn main() {
    let a: &str = "123456";
    let s: String = a.to_owned(); // 将引用转换为所有权实例
    println!("{}", a);
    println!("{}", a);
    println!("{}", s);

    let s = "I am a superman.";
    let s1: String = String::from(s); // 使用 String 的from构造器
    let s2: String = s.to_string(); // 使用 to_string() 方法;只是转成字符串而已，这两个用法重叠，但是不完全相同。
    let s3: String = s.to_owned(); // 使用 to_owned() 方法
    // 是更通用的，目的就是在只能拿到引用的情况下获得所有权（通过克隆一份资源）。
    //
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
