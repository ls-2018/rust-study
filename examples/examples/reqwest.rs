#![allow(unused)]

use axum::http::StatusCode;
use reqwest::multipart::{Form, Part};

async fn post() -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let files = Part::bytes(b"data")
        .file_name("Cargo.toml")
        .mime_str("text/plain")?;
    let form = Form::new().part("file", files);

    let res = client
        .post("http://127.0.0.1:8080/api/upload")
        .header("Authorization", "Bearer ".to_string())
        // .header("Content-Type", "application/json")
        .multipart(form)
        // .body(body);
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let ret: Vec<String> = res.json().await?;
    println!("{:?}", ret);
    Ok(())
}
fn main() {}
