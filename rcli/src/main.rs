use clap::Parser;
use rcli::{CmdExecutor, Opts, init_logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = init_logger();
    // tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await?;
    Ok(())
}
