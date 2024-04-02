use clap::{CommandFactory, Parser, Subcommand};

use crate::commands::import::import;
use crate::commands::install::install;
use crate::commands::set::set;

mod commands;
mod errors;
mod protos;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.commands {
        Commands::Install { build, arch, version } => install(build, arch, version).await,
        Commands::Set { build, arch, version } => set(build, arch, version).await,
        Commands::Import { path } => import(path).await
    }.unwrap_or_else(|err|
        Args::command().error(err.error().kind, err.error().message).exit()
    );
}

#[derive(Parser)]
#[command(
    bin_name = "jara",
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None,
)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) commands: Commands
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Install a JDK
    Install {
        build: String,
        arch: String,
        version: String
    },
    /// Set current JDK
    Set {
        build: String,
        arch: String,
        version: String
    },
    /// Import JDK
    Import {
        path: String
    }
}
