use proto_builder_trait::tonic::BuilderAttributes;
use std::error::Error;
use std::fs;

static OUT_DIR: &str = "src/proto-gen";

fn main() -> Result<(), Box<dyn Error>> {
    let protos = ["proto/basic/basic.proto", "proto/query.proto"];

    fs::create_dir_all(OUT_DIR).unwrap();
    tonic_build::configure()
        .build_server(true)
        .out_dir(OUT_DIR)
        .with_serde(&["User"], true, true, Some(&[r#"#[serde(rename_all = "camelCase")]"#]))
        .with_sqlx_from_row(&["User"], None)
        .with_derive_builder(&["QueryRequest", "User"], None)
        // .with_field_attributes(
        //     &["GoodbyeRequest.ids","User.email"],
        //     &[r#"#[builder(setter(into))]"#],
        // )
        .compile_protos(&protos, &["proto/"])?;

    Ok(())
}
