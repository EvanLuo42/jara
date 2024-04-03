use clap::{CommandFactory, Parser, Subcommand};
use simple_home_dir::home_dir;
use tokio::fs;

use crate::commands::import::import;
use crate::commands::install::install;
use crate::commands::set::set;
use crate::commands::versions::versions;
use crate::errors::JaraErrors;

mod commands;
mod errors;
mod protos;

#[tokio::main]
async fn main() {
    fs::create_dir_all(format!("{}/.jara", home_dir().unwrap().display())).await
        .unwrap_or_else(|err| {
            let error = JaraErrors::Other { message: err.to_string() };
            Args::command().error(error.error().kind, error.error().message).exit()
        });
    let args = Args::parse();
    match args.commands {
        Commands::Install { build, arch, version } =>
            install(build, arch, version).await,
        Commands::Set { build, arch, version } =>
            set(build, arch, version).await,
        Commands::Import { path } => import(path).await,
        Commands::Versions => versions().await
    }.unwrap_or_else(|err|
        Args::command()
            .error(err.error().kind, err.error().message)
            .exit()
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
    },
    /// List all imported & installed versions
    Versions
}
