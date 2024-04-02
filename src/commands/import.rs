use std::io::ErrorKind;
use colored::Colorize;
use simple_home_dir::home_dir;

use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

use crate::errors::{JaraErrors, JaraResult};
use crate::protos::versions::{Arch, Build, Version, Versions};

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

    let mut versions_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}/.jara/versions", home_dir().unwrap().display()))
        .await
        .map_err(|err| match err.kind() {
            ErrorKind::PermissionDenied => JaraErrors::PermissionDenied,
            ErrorKind::NotFound => JaraErrors::VersionsFileNotFound,
            kind => JaraErrors::Other { message: kind.to_string() }
        })?;

    let mut content = String::new();
    BufReader::new(&mut versions_file)
        .read_to_string(&mut content)
        .await
        .map_err(|err| JaraErrors::Other { message: err.to_string() })?;
    let mut versions: Versions = if content.is_empty() {
        Versions {
            versions: Vec::new()
        }
    } else {
        toml::from_str(content.as_str())
            .map_err(|err| JaraErrors::Other { message: err.to_string() })?
    };

    let try_find_version = maps
        .iter()
        .find(|map| map.0 == String::from("JAVA_VERSION"));
    if let None = try_find_version {
        return Err(JaraErrors::InvalidJDK)
    }
    let version = try_find_version
        .unwrap()
        .clone()
        .1
        .replace("\"", "");
    println!("{}: {}", "Version".green().bold(), version);

    let try_find_build = maps
        .iter()
        .find(|map| map.0 == String::from("IMPLEMENTOR"));
    if let None = try_find_build {
        return Err(JaraErrors::InvalidJDK)
    }
    let build: Build = try_find_build
        .unwrap()
        .clone()
        .1
        .replace("\"", "")
        .parse()?;
    println!("{}: {}", "Build".green().bold(), build);

    let try_find_arch = maps
        .iter()
        .find(|map| map.0 == String::from("OS_ARCH"));
    if let None = try_find_arch {
        return Err(JaraErrors::InvalidJDK)
    }
    let arch: Arch = try_find_arch
        .unwrap()
        .clone()
        .1
        .replace("\"", "")
        .parse()?;
    println!("{}: {}", "Arch".green().bold(), arch);

    let version = Version {
        build,
        version,
        arch,
    };
    if versions.versions.iter().find(|version|
        version.build == build && version.arch == arch
            && version.version.eq(&version.version)
    ).is_some() {
        return Err(JaraErrors::VersionConflict)
    }

    versions.versions.push(version);

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
