## 过程宏


### 派生宏
```
#[derive(Debug, thiserror::Error, ToErrorInfo)]


[lib]
proc-macro = true


#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_to_error_info(input: TokenStream) -> TokenStream {

```
