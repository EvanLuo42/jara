use crate::errors::JaraResult;
use crate::protos::versions::{Arch, Build};

pub(crate) trait JDKImplementor {
    async fn fetch_releases() -> JaraResult<Release>;
    async fn download_release() -> JaraResult<()>;
}

pub(crate) struct Release {
    version: String,
    arch: Arch,
    build: Build
}
