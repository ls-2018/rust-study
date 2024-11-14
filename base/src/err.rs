use anyhow::anyhow;
use std::fs::File;
use std::io;
use std::io::{Error, Read};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

fn xxx() -> Result<(), DataStoreError> {
    struct MyError;
    type Result<String> = std::result::Result<String, MyError>;
    // 假设这里有一个 I/O 操作
    let result = std::fs::read_to_string("nonexistent_file.txt");

    // 如果 I/O 操作失败，使用 `?` 将错误转换为我们的自定义错误类型
    let contents = result.map_err(DataStoreError::Disconnect)?;
    anyhow!("Missing attribute: {}", "missing");
    // x.into() // Error -> DataStoreError
    // 假设我们基于某些条件引发一个自定义错误
    if contents.is_empty() { Err(DataStoreError::Redaction("".to_string())) } else { Ok(()) }
}

struct MyError(String); // 用newtype方法定义了一个新的错误类型

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError(format!("{}", err))
    }
}

fn read_file() -> Result<String, MyError> {
    xxx().expect("");
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    let x = Error::new(io::ErrorKind::Other, "contents");
    let a: MyError = x.into();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}
