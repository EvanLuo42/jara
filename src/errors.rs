use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use clap::error::ErrorKind;

pub(crate) type JaraResult<T> = Result<T, JaraErrors>;

pub(crate) struct JaraError {
    pub(crate) kind: ErrorKind,
    pub(crate) message: String
}

impl JaraError {
    pub(crate) fn new(kind: ErrorKind, message: String) -> JaraError {
        JaraError {
            kind,
            message
        }
    }
}

pub(crate) fn map_other_errors(error: impl Error) -> JaraErrors {
    JaraErrors::Other { message: error.to_string() }
}

#[derive(Debug)]
pub(crate) enum JaraErrors {
    InvalidJDK,
    PermissionDenied,
    VersionsFileNotFound,
    UnsupportedBuild,
    UnsupportedArch,
    VersionConflict,
    VersionNotFound,
    Other { message: String }
}

impl JaraErrors {
    pub(crate) fn error(&self) -> JaraError {
        match self {
            JaraErrors::InvalidJDK => JaraError::new(
                ErrorKind::InvalidValue, "Not a valid JDK.".into()
            ),
            JaraErrors::PermissionDenied => JaraError::new(
                ErrorKind::Io, "Permission denied.".into()
            ),
            JaraErrors::VersionsFileNotFound => JaraError::new(
                ErrorKind::Io, "Versions file not found.".into()
            ),
            JaraErrors::UnsupportedArch => JaraError::new(
                ErrorKind::Io, "Unsupported arch.".into()
            ),
            JaraErrors::UnsupportedBuild => JaraError::new(
                ErrorKind::Io, "Unsupported build.".into()
            ),
            JaraErrors::VersionConflict => JaraError::new(
                ErrorKind::ValueValidation, "Version conflict.".into()
            ),
            JaraErrors::VersionNotFound => JaraError::new(
                ErrorKind::ValueValidation, "Version not found.".into()
            ),
            JaraErrors::Other { message } => JaraError::new(
                ErrorKind::Io, message.clone()
            ),
        }
    }
}

impl Display for JaraErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error().message)
    }
}

impl Error for JaraErrors {

}
