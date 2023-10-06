use std::env;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use super::{ConfigError, Result};

pub fn load(path: PathBuf) -> Result<Value> {
    let path = path
        .canonicalize()
        .map_err(|source| ConfigError::ResolvePath {
            source,
            path,
            cwd: env::current_dir(),
        })?;

    let file = fs::read(&path).map_err(|source| ConfigError::FileOpenFailed {
        source,
        path: path.to_path_buf(),
    })?;
    serde_json::from_slice(&file).map_err(|source| ConfigError::InvalidJson { source, path })
}
