use std::io::ErrorKind;

use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

use crate::errors::{JaraErrors, JaraResult};
use crate::protos::versions::{Arch, Build, Version, Versions};

pub(crate) async fn import(path: String) -> JaraResult<()> {
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
        let key_value = line.split("=").collect::<Vec<String>>();
        if key_value.len() != 2 {
            continue;
        }
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
        .open("~/.jara/versions")
        .await
        .map_err(|err| match err.kind() {
            ErrorKind::PermissionDenied => JaraErrors::PermissionDenied,
            ErrorKind::NotFound => JaraErrors::VersionsFileNotFound,
            kind => JaraErrors::Other { message: kind.to_string() }
        })?;

    let mut content = String::new();
    BufReader::new(versions_file)
        .read_to_string(&mut content)
        .await
        .map_err(|err| JaraErrors::Other { message: err.to_string() })?;
    let mut versions: Versions = toml::from_str(content.as_str())
        .map_err(|err| JaraErrors::Other { message: err.to_string() })?;

    let try_find_version = maps
        .iter()
        .find(|map| map.0 == String::from("JAVA_VERSION"));
    if let None = try_find_version {
        return Err(JaraErrors::InvalidJDK)
    }
    let version = try_find_version.unwrap().clone().1;

    let try_find_build = maps
        .iter()
        .find(|map| map.0 == String::from("IMPLEMENTOR"));
    if let None = try_find_build {
        return Err(JaraErrors::InvalidJDK)
    }
    let build: Build = try_find_build.unwrap().clone().1.parse()?;

    let try_find_arch = maps
        .iter()
        .find(|map| map.0 == String::from("OS_ARCH"));
    if let None = try_find_arch {
        return Err(JaraErrors::InvalidJDK)
    }
    let arch: Arch = try_find_arch.unwrap().clone().1.parse()?;

    let version = Version {
        build,
        version,
        arch,
    };
    if versions.versions.iter().find(|version|
        version.build == build && version.arch == arch
            && version.version.eq(version)
    ).is_some() {
        return Err(JaraErrors::VersionConflict)
    }

    versions.versions.push(version);

    let serialized_versions = toml::to_string(&versions).map_err(|err|
        JaraErrors::Other { message: err.to_string() }
    )?;

    Ok(())
}
