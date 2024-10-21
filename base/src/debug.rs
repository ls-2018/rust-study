use std::{
    fmt::{Debug, Formatter},
    result,
};

// Ord

// PartialEq 和其他判等
// Eq 和自己判等
// PartialOrd 和其他排序
// Ord 和自己排序

// #[derive(Debug, PartialEq, Eq, PartialOrd,Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        result::Result::Ok(())
    }
}
