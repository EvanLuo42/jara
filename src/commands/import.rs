use std::io::ErrorKind;

use colored::Colorize;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::commands::{read_versions_file, write_versions_file};
use crate::errors::{JaraErrors, JaraResult};
use crate::protos::versions::Version;

pub(crate) async fn import(path: String) -> JaraResult<()> {
    println!("{} {}/release", "Reading".green().bold(), path);
    let release = File::open(format!("{}/release", path))
        .await
        .map_err(|err| match err.kind() {
            ErrorKind::PermissionDenied => JaraErrors::PermissionDenied,
            ErrorKind::NotFound => JaraErrors::InvalidJDK,
            kind => JaraErrors::Other { message: kind.to_string() }
        })?;
    let mut lines = BufReader::new(release).lines();
    let mut maps = Vec::<(String, String)>::new();

    while let Some(line) = lines.next_line().await.map_err(|err|
        JaraErrors::Other { message: err.to_string() }
    )? {
        if !line.contains("=") {
            continue;
        }
        let key_value = line.split("=").collect::<Vec<&str>>();
        if key_value.len() != 2 {
            continue;
        }
        let key_value: Vec<String> = key_value
            .iter()
            .map(|x| x.to_string())
            .collect();
        let map = (
            key_value.get(0).unwrap().clone(),
            key_value.get(1).unwrap().clone()
        );
        maps.push(map);
    }

    let mut versions = read_versions_file().await?;

    let version = maps
        .iter()
        .find(|map| map.0 == String::from("JAVA_VERSION"))
        .ok_or(JaraErrors::InvalidJDK)?
        .1
        .replace("\"", "");
    println!("{}: {}", "Version".green().bold(), version);

    let build = maps
        .iter()
        .find(|map| map.0 == String::from("IMPLEMENTOR"))
        .ok_or(JaraErrors::InvalidJDK)?
        .1
        .replace("\"", "")
        .parse()?;
    println!("{}: {}", "Build".green().bold(), build);

    let arch = maps
        .iter()
        .find(|map| map.0 == String::from("OS_ARCH"))
        .ok_or(JaraErrors::InvalidJDK)?
        .1
        .replace("\"", "")
        .parse()?;
    println!("{}: {}", "Arch".green().bold(), arch);

    let version = Version {
        build,
        version,
        arch,
        path,
    };
    if versions
        .versions
        .iter()
        .any(|_version| *_version == version) {
        return Err(JaraErrors::VersionConflict)
    };

    versions.versions.push(version);

    write_versions_file(versions).await?;

    Ok(())
}
