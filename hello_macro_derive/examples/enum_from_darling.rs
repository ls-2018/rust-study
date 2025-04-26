#![allow(dead_code)]
use macros::EnumFromDarling;

#[derive(Debug, EnumFromDarling)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[derive(Debug)]
struct DirectionUp {
    speed: i32,
}

fn main() {
    let up: Direction = DirectionUp { speed: 12 }.into();
    let left: Direction = 10.into();
    println!("{:?}, {:?}", up, left);
}
