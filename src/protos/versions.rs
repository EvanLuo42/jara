use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::errors::JaraErrors;

#[derive(Deserialize, Serialize)]
pub(crate) struct Versions {
    pub(crate) versions: Vec<Version>
}

pub(crate) struct Version {
    pub(crate) build: Build,
    pub(crate) version: String,
    pub(crate) arch: Arch
}

#[derive(Eq, PartialEq)]
pub(crate) enum Build {
    Zulu
}

impl FromStr for Build {
    type Err = JaraErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Azul Systems, Inc." => Ok(Build::Zulu),
            _ => Err(JaraErrors::UnsupportedBuild)
        }
    }
}

#[derive(Eq, PartialEq)]
pub(crate) enum Arch {
    Arm64, Amd64
}

impl FromStr for Arch {
    type Err = JaraErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aarch64" => Ok(Arch::Arm64),
            _ => Err(JaraErrors::UnsupportedBuild)
        }
    }
}