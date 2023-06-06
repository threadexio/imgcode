use anyhow::Result;
use clap::Parser;

mod util;

mod decode;
mod encode;

pub(self) mod command_prelude {
    pub(crate) use super::CliArgs;
    pub(crate) use crate::util;

    pub use anyhow::{bail, Context, Result};
}

#[derive(Debug, clap::Subcommand)]
enum CliCommands {
    Decode(decode::Args),
    Encode(encode::Args),
}

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[clap(subcommand)]
    command: CliCommands,
}

fn try_main() -> Result<()> {
    let global_args = CliArgs::parse();

    match &global_args.command {
        CliCommands::Decode(cmd_args) => decode::command(&global_args, cmd_args),
        CliCommands::Encode(cmd_args) => encode::command(&global_args, cmd_args),
    }
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {e:#}");
    }

    std::process::exit(-1);
}
