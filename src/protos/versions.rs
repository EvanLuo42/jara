use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::errors::JaraErrors;

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Versions {
    pub(crate) versions: Vec<Version>
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub(crate) struct Version {
    pub(crate) build: Build,
    pub(crate) version: String,
    pub(crate) arch: Arch,
    pub(crate) path: String
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Deserialize, Serialize)]
pub(crate) enum Build {
    Zulu
}

impl FromStr for Build {
    type Err = JaraErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Azul Systems, Inc." | "zulu" => Ok(Build::Zulu),
            _ => Err(JaraErrors::UnsupportedBuild)
        }
    }
}

impl Display for Build {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Build::Zulu => write!(f, "Zulu")
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Deserialize, Serialize)]
pub(crate) enum Arch {
    Arm64, Amd64
}

impl FromStr for Arch {
    type Err = JaraErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aarch64" | "arm64" => Ok(Arch::Arm64),
            _ => Err(JaraErrors::UnsupportedArch)
        }
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::Amd64 => write!(f, "Amd64"),
            Arch::Arm64 => write!(f, "Arm64"),
        }
    }
}
