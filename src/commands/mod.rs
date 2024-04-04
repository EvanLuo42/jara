use std::io::ErrorKind;

use colored::Colorize;
use simple_home_dir::home_dir;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

use crate::errors::JaraErrors;
use crate::protos::versions::Versions;

pub(crate) mod install;
pub(crate) mod set;
pub(crate) mod import;
pub(crate) mod versions;

pub(crate) async fn read_versions_file() -> Result<Versions, JaraErrors> {
    let mut versions_file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(format!("{}/.jara/versions.toml", home_dir().unwrap().display()))
        .await
        .map_err(|err| match err.kind() {
            ErrorKind::PermissionDenied => JaraErrors::PermissionDenied,
            kind => JaraErrors::Other { message: kind.to_string() }
        })?;

    let mut content = String::new();
    BufReader::new(&mut versions_file)
        .read_to_string(&mut content)
        .await
        .map_err(|err| JaraErrors::Other { message: err.to_string() })?;
    if content.is_empty() {
        return Ok( Versions {
            versions: Vec::new()
        })
    }

    toml::from_str(content.as_str())
        .map_err(|err| JaraErrors::Other { message: err.to_string() })
}

pub(crate) async fn write_versions_file(versions: Versions) -> Result<(), JaraErrors> {
    let versions_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/.jara/versions.toml", home_dir().unwrap().display()))
        .await
        .map_err(|err| match err.kind() {
            ErrorKind::PermissionDenied => JaraErrors::PermissionDenied,
            ErrorKind::NotFound => JaraErrors::VersionsFileNotFound,
            kind => JaraErrors::Other { message: kind.to_string() }
        })?;
    let serialized_versions = toml::to_string(&versions).map_err(|err|
        JaraErrors::Other { message: err.to_string() }
    )?;

    let mut writer = BufWriter::new(versions_file);
    writer.write(serialized_versions.as_bytes()).await.map_err(|err|
        JaraErrors::Other { message: err.to_string() }
    )?;
    writer.flush().await.map_err(|err|
        JaraErrors::Other { message: err.to_string() }
    )?;
    println!("{}", "Successfully imported version!".green().bold());

    Ok(())
}
