use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn dir_error(msg: &str, dir: &Path) -> String {
    format!("{}: {}", msg, dir.to_str().unwrap_or(""))
}

#[derive(thiserror::Error, Debug)]
pub enum CreateDirError {
    #[error("Failed to create new directory: {0}")]
    FailedToCreate(PathBuf),
    #[error("Expected to be directory: {0}")]
    ExpectedDirectory(PathBuf),
}

pub fn create_or_get_dir(directory: &Path) -> Result<PathBuf, CreateDirError> {
    if !directory.exists() {
        fs::create_dir(directory).map_err(|_| CreateDirError::FailedToCreate(directory.to_owned()))?;
    }
    if !directory.is_dir() {
        return Err(CreateDirError::ExpectedDirectory(directory.to_owned()));
    }

    fs::canonicalize(directory).map_err(|_| CreateDirError::FailedToCreate(directory.to_owned()))
}
