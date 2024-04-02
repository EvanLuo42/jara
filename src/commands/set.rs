use simple_home_dir::home_dir;
use tokio::fs::{remove_file, symlink};

use crate::commands::read_versions_file;
use crate::errors::{JaraErrors, JaraResult, map_other_errors};
use crate::protos::versions::Version;

pub(crate) async fn set(
    build: String,
    arch: String,
    version: String
) -> JaraResult<()> {
    let versions = read_versions_file().await?;

    let expect_version = Version {
        build: build.parse()?,
        arch: arch.parse()?,
        version,
        path: String::new()
    };

    let version = versions.versions
        .iter()
        .find(|_version|
            _version.version == expect_version.version &&
                _version.build == expect_version.build &&
                _version.arch == expect_version.arch
        )
        .ok_or(JaraErrors::VersionNotFound)?;

    let _ = remove_file(format!("{}/.jara/bin", home_dir().unwrap().display()))
        .await;
    symlink(
        format!("{}/bin", version.path),
        format!("{}/.jara/bin", home_dir().unwrap().display())
    ).await.map_err(map_other_errors)?;

    Ok(())
}