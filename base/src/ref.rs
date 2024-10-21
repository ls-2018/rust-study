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
