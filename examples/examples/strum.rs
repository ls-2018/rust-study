#![allow(unused)]

use anyhow::Result;
use serde::Serialize;
use strum::{Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr, VariantNames};
#[derive(Display, Debug, Serialize)]
enum Color {
    #[strum(serialize = "redred", to_string = "red")] // to_string 优先级更高
    Red,
    Green {
        range: usize,
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    },
}

// EnumString           Converts strings to enum variants based on their name.
// Display              Converts enum variants to strings
// FromRepr             Convert from an integer to an enum.
// AsRefStr             Implement AsRef<str> for MyEnum
// IntoStaticStr        Implements From<MyEnum> for &'static str on an enum
// EnumIter             Creates a new type that iterates of the variants of an enum.
// EnumProperty         Add custom properties to enum variants.
// EnumMessage          Add a verbose message to an enum variant.
// EnumDiscriminants    Generate a new type with only the discriminant names.
// EnumCount            Add a constant usize equal to the number of variants.
// VariantArray         Adds an associated VARIANTS constant which is an array of all enum discriminants
// VariantNames         Adds an associated VARIANTS constant which is an array of discriminant names
// EnumTable            Experimental, creates a new type that stores an item of a specified type for each variant of the enum.
#[derive(Debug, EnumString, EnumCount, EnumDiscriminants, EnumIter, EnumIs, IntoStaticStr, VariantNames)]
pub enum MyEnum {
    A,
    B(String),
    C,
    D,
}

fn main() -> Result<()> {
    println!("{:?}", MyEnum::VARIANTS);
    MyEnum::iter().for_each(|v| println!("{:?}", v));
    println!("total: {:?}", MyEnum::COUNT);

    let my_enum: MyEnum = MyEnum::B("hello".to_string());
    println!("{:?}", my_enum.is_b());
    let s: &'static str = my_enum.into();
    println!("{}", s);

    let red = Color::Red;
    let green = Color::Green { range: 10 };
    let blue = Color::Blue(20);
    let yellow = Color::Yellow;
    let purple = Color::Purple { sat: 30 };

    println!("red: {}, green: {}, blue: {}, yellow: {}, purple: {}", red, green, blue, yellow, purple);

    let red_str = serde_json::to_string(&red)?;
    println!("{}", red_str);

    Ok(())
}
