use crate::r#struct::*;
use csv::Reader;
use std::fs;

pub fn process_csv(input: String, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;

    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    // let res = match format {
    //     OutputFormat::Json => serde_json::from_reader(&ret)?,
    //     OutputFormat::Yaml => serde_yaml::from_reader(&ret)?,
    // };

    fs::write(output, content)?;
    Ok(())
}
