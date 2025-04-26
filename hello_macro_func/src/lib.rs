use proc_macro::TokenStream;

use std::str::FromStr;

#[proc_macro]
pub fn func(_input: TokenStream) -> TokenStream {
    let output = format!("fn generated_function(a: i32, b: i32) -> i32 {{ a + b }}");
    TokenStream::from_str(&*output).unwrap()
}
