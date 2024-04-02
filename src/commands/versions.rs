use colored::Colorize;
use crate::commands::read_versions_file;
use crate::errors::JaraResult;

pub(crate) async fn versions() -> JaraResult<()> {
    let versions = read_versions_file().await?;
    for version in versions.versions {
        println!(
            "{}/{}-{} \n{}\n",
            version.build.to_string().to_ascii_lowercase().blue().bold(),
            version.version,
            version.arch.to_string().to_ascii_lowercase(),
            version.path
        );
    }
    Ok(())
}