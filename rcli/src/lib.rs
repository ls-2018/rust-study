mod cli;
mod logger;
mod process;
mod r#struct;
mod utils;
// mod server;
// mod utils;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use logger::*;
pub use process::*;
pub use r#struct::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
