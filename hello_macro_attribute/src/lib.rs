use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // attr 表示属性的输入
    // item 表示应用该属性的代码块
    let function_name = attr.to_string();
    let mut result = item.to_string();

    result.push_str(&format!("fn {}() {{", function_name));
    result.push_str("println!(\"属性宏\"); }");

    result.parse().unwrap()
}
