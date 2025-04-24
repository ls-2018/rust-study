use proc_macro::TokenStream;

#[proc_macro_attribute] // 属性宏
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let x = "";
    x.parse().unwrap()
}
