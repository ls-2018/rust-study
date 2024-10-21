// 一种类型转换成另一种类型,但是要在引用符号 &、点号操作符 .
// 或其他智能指针的触发下才会产生转换。

// 还有 &Vec<T> 可以自动转换为 &[T]，也是因为 Vec[T] 实现了 Deref。

// 比如标准库里最常见的 &String 可以自动转换到 &str ，就是因为 String 类型实现了
// Deref trait。 还有 &Vec 可以自动转换为 &[T]，也是因为 Vec[T] 实现了 Deref。

use std::fmt::Display;
use std::ops::*;
struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("deref");
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        println!("deref_mut");
        &mut self.elements[self.current]
    }
}

#[test]
fn x() {
    let a = vec!['x', 'y', 'z'];
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };

    // 因为`Selector`实现了`Deref`，所以可以使用`*`运算符来引用它的当前元素
    assert_eq!(*s, 'z');
    // 通过隐式解引用直接在`Selector`上使用`char`的方法断言'z'是字母
    assert!(s.is_alphabetic());
    // 通过对此`Selector`的引用目标赋值，把'z'改成了'w'
    *s = 'w';
    assert_eq!(s.elements, ['x', 'y', 'w']);
    s.deref(); // &char
    s.deref_mut(); //  mut &char
    show_it(&s);
    show_it(s.deref());
    show_it(&*s);
    show_it(&s.deref_mut());
    show_it_generic(s.deref()); // &char
}

fn show_it(thing: &char) {
    println!("{}", thing);
}
fn show_it_generic<T: Display>(thing: T) {
    println!("{}", thing);
}
