#![allow(unused_must_use)]
#![allow(unused_macros)]

#[macro_export]
macro_rules! print_message {
    () => {
        println!("声明宏");
    };
        ($x:expr, $y:expr) => {
        $x + $y
    };
      ($x:expr, $($rest:expr),*) => {
        $x + print_message!($($rest),*)
    };
}
