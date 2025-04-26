```
#[derive(Eq, PartialEq, Ord, PartialOrd)]

Clone, 用来从 &T 创建副本 T。
Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。

#[derive(Clone, Copy)]//
# 数字实现了 Copy
Arc, Rc

Locks, Send, RefCell, Async/Sync/Send, RwLock
```



```
env CARGO_LOG=cargo::core::resolver=trace cargo update
cargo tree --workspace --target all --all-features --edges features --invert rand
cargo tree --workspace --target all --all-features --invert rand
```





```
#[derive(Copy, Debug)]
struct Flags {
    on: bool,
    level: u8,
}

#[derive(Clone, Debug)]
struct Flags2 {
    on: bool,
    name: String,
}

fn main() {
    let a = Flags {
        on: false,
        level: 1,
    };
    let b = a;
    println!("An integer: {:?}", a);
    println!("An integer: {:?}", b);

    let an_integer = Flags2 {
        on: false,
        name: String::from("an_integer"),
    };
    let copied_integer = an_integer.clone();
    println!("An integer: {:?}", copied_integer);
    println!("An integer: {:?}", an_integer);
}

```



```
impl Deref for String {
    type Target = str;

    fn deref(&self) -> &str {
    }
}

&String  -> &str
```



```
std::mem::size_of::<bool>();
```



```
pub trait From<T: ?Sized>: Sized {
    fn from(value: T) -> Self;
}


impl Into<WrappingU32> for u32 {}
```



```
pub trait Add<TYPE = Self> {
    type Output;
    fn add(self, rhs: TYPE) -> Self::Output;
}
```



```
// Slightly simplified
pub trait Index<Idx>
{
    type Output;

    // Required method
    fn index(&self, index: Idx) -> &Self::Output;
}

// Slightly simplified
pub trait IndexMut<Idx>: Index<Idx>
{
    // Required method
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```



```
let turbo_parsed: i32 = "10".parse::<i32>().unwrap();
```



```
let (sender, receiver) = std::sync::mpsc::channel();// multi send,single rev
let (sender, receiver) = std::sync::mpsc::sync_channel(10);
```



```
// 通过这个属性屏蔽警告。
#[allow(non_camel_case_types)]
```



```
.into_iter() 会消耗集合。在每次迭代中，集合中的数据本身会被提供。
.iter_mut()  借用集合中的每个元素，从而允许集合被就地修改。
.iter() - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。

Iterator()
```



```
let mut mut_value = 6;
// *mut_value = 1; //can't dereferenced
let ref mut a = mut_value;
*a = 1;
println!("{}", mut_value);
```



```
match pair {
    (x, y) if x == y =>
    n @ 1  ..= 12 =>
    Some(n @ 42) =>
```



```
// #[derive(PartialEq)]
enum Foo {
    Bar,
}

fn main() {
    let a = Foo::Bar;

    if let Foo::Bar = a {
        println!("b is foobar");
    }
    if Foo::Bar == a { // 没实现 #[derive(PartialEq)] 会报错
        println!("b is foobar");
    }
}



let sum_of_squared_odd_numbers: u32 =
    (0..).map(|n| n * n)             // 所有自然数取平方
         .take_while(|&n| n < upper) // 取小于上限的
         .filter(|&n| is_odd(n))     // 取奇数
         .fold(0, |sum, i| sum + i); // 最后加起来

```



```
// 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
// `path` 必须是父模块（parent module）或祖先模块（ancestor module）

// 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
// 使用 `pub(super)` 语法定义的函数只在父模块中可见。
// `pub(crate)` 使得函数只在当前 crate 中可见 项可以在同一个 crate 中的任何地方访问
```



```
rustc --crate-type=lib split.rs
rustc executable.rs --extern split=split.rlib --edition=2018 && ./executable
--bin my_other_bin

#[cfg(not(target_os = "linux"))]
if cfg!(target_os = "linux") {

#[cfg(some_condition)]
rustc --cfg some_condition custom.rs && ./custom




```



```
fn random_animal(random_number: f64) -> Box<dyn Animal> {

use std::ops;
impl ops::Add {}

<Form as AgeWidget>::get(&form);
```



```
// 这是一个简单的声明宏，名为 `say_hello`。
macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数。
    () => (
        // 此宏将会展开成这个代码块里面的内容。
        println!("Hello!");
    )
      ($expression:expr) => (
      // `stringify!` 把表达式*原样*转换成一个字符串。
      println!("{:?} = {:?}",
               stringify!($expression),// 字符串
               $expression)// 值
    )
}

fn main() {
    // 这个调用将会展开成 `println("Hello");`!
    say_hello!()
}
----
block：一个块，即被{}包裹的语句或者表达式的块；
expr：表达式；
ident：标识符（包括keywords）
item：一个项，比如函数、struct、module、impl等；
lifetime：生命周期，比如'a、'static等；
literal：一个字面量，比如"3.14"、"hello world"等；
meta：元项，#[...]或者#![...]这俩属性的内部内容，比如#[derive("test")]里的derive("test")；
pat：一个模式(pattern)；
path：一个路径，比如foo， ::std::mem::replace， transmute::<_, int>等；
stmt：语句；
tt：一个简单的token tree；
ty：一个类型；
vis：一个可能为空可见性限定符(不知道啥意思。。。原文：a possible empty visibility qualifier)，比如pub， pub(in crate)等；
---
// 参数不需要使用逗号隔开。
// 参数可以任意组合！
($left:expr; and $right:expr) =>
// ^ 每个分支都必须以分号结束。

macro_rules! calculate {
    // 单个 `eval` 的模式
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // 递归地拆解多重的 `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! { // 妈妈快看，可变参数的 `calculate!`！
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}

```



```
fn combine_vacs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter())
}
```



```
使用 ? 解开 Option

Option 有一个内置方法 map()，这个组合算子可用于 Some -> Some 和 None -> None 这样的简单映射。多个不同的 map() 调用可以串起来，这样更加灵活。

Some(food).and_then(have_recipe)

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    opt.map_or(Result::Ok(None), |r| r.map(Option::Some))
}
```





```
在rust中，宏有两种。

declarative macros也就是声明宏，关键字macro_rules!
procedural macros也就是过程宏,它有以下三类
派生(#[derive])宏，自定义派生的属性,作用于structs和enum类型。
类属性(Attribute-like)宏，要不就叫属性宏算了。。可自定义属性作用于任何的项中。
类函数(Function-like)宏，这个就叫函数宏算了。。看起来像是函数调用但是操作将它作为参数的tokens。
```
