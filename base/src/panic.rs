use std::fmt;

#[test]
fn x() {
    let x = xa();
    if x.is_err() {
        println!("error: {:?}", x.err()); // 警告：未使用的结果
    }
}

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

// 错误应该是可打印的
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// 错误应该实现std::error::Error特型，但使用Error各个方法的默认定义就够了
impl std::error::Error for JsonError {}

// use thiserror::Error;
// #[derive(Error)] 指令会让 thiserror 生成前面展示过的代码，这可以节省大量的时间和精力。
// #[derive(Error, Debug)]
// #[error(" (, )")]
// pub struct JsonError {
//     message: String,
//     line: usize,
//     column: usize,
// }

fn xa() -> Result<u64, JsonError> {
    return Err(JsonError {
        message: "expected ']' at end of array".to_string(),
        line: 1,
        column: 2,
    });
}
