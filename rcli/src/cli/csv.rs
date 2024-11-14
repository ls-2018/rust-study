use super::verify_file;
use crate::r#struct::OutputFormat;
use crate::{CmdExecutor, process};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: String,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// pub struct Arg {

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl CmdExecutor for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{}", self.output);
        process::process_csv(self.input, self.output, self.format)
    }
}
