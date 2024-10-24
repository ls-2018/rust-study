use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    skills: Vec<String>,
}

// tracing 会自动为函数创建一个 span，span 名跟函数名相同，在输出的信息中还会自动带上函数参数。

#[instrument]
async fn instrument_test() -> User {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };
    user
}

async fn instrument_test2() -> User {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };
    user
}
#[tokio::main]
async fn main() -> Result<()> {
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);
    tracing_subscriber::registry().with(console).init();

    let res = instrument_test().await;
    info!("{:?}", res);
    let res = instrument_test2().await;
    info!("{:?}", res);
    Ok(())
}
