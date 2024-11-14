#![allow(unused)]

// 声明这个 crate 为 lib，是全局性的属性
mod _async;
mod add;
mod async_client;
mod barrier;
mod base;
mod class;
mod debug;
mod display;
mod env;
mod err;
mod file;
mod r#for;
mod from;
mod r#if;
mod io;
mod iter;
mod r#match;
mod module;
mod mutex;
mod panic;
mod parse;
mod pbj;
mod random;
mod rc;
mod r#ref;
mod reg;
mod safe;
mod str;
mod r#trait;
mod trait_asref;
mod trait_bibao;
mod trait_copy;
mod trait_default;
mod trait_deref;
mod trait_drop;
mod r#type;
mod vec;
mod vegetables;
mod web;
mod width_ref;

#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unreachable_patterns)]
#[allow(unused_mut)]
#[allow(non_snake_case)]
#[allow(unused_must_use)]
#[allow(dead_code)]
#[allow(unused_assignments)]
pub use file::*;
