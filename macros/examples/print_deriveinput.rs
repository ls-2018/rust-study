#![allow(unused)]
use anyhow::Result;
use proc_macro2::TokenStream as TokenStreamV2;
use std::str::FromStr;
use syn::DeriveInput;

fn parse2_for_proc_macro2() -> Result<()> {
    let input = proc_macro2::TokenStream::from_str(
        "
enum Direction {
    A(Option<u32>),
    Down(u32),
    Right(DirectionUp),
    X,
}
",
    )
    .expect("--->");
    // println!("{:?}", input);
    let di = syn::parse2::<DeriveInput>(input)?;
    println!("{:#?}", di);
    Ok(())
}

fn main() {
    parse2_for_proc_macro2().expect("---");
}

struct DirectionUp {
    speed: u32,
}

enum Direction {
    A(Option<u32>),
    Down(u32),
    Right(DirectionUp),
    X,
}
