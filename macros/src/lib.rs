mod macros;
use proc_macro::TokenStream;
use syn::DeriveInput; // # syn: Rust语法解析器能够用于解析TokenStream

// #[proc_macro_derive(EnumFrom)]：
// 这是一个属性宏，它告诉 Rust 编译器 derive_enum_from 函数是一个过程宏，用于处理 #[derive(EnumFrom)] 宏。
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", input);
    macros::process_enum_from(input).into()
}

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", input);

    macros::process_enum_from_darling(input).into()
}

#[proc_macro_derive(AutoDeref, attributes(deref, deref2))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);

    macros::process_auto_deref(input).into()
}

#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn derive_auto_debug(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", input);
    macros::process_auto_debug(input).into()
}

#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_to_error_info(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    macros::process_error_info(input).into()
}
