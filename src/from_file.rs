use std::env;
use std::fs;
use std::path::PathBuf;

use failure::{Error, ResultExt};
use serde_json::Value;

use super::ConfigError;

pub fn load(path: PathBuf) -> Result<Value, Error> {
    let path = path
        .canonicalize()
        .with_context(|_| ConfigError::ResolvePath {
            path,
            cwd: env::current_dir(),
        })?;

    Ok(actual_load(&path).with_context(|_| ConfigError::LoadingFile { path })?)
}

fn actual_load(path: &PathBuf) -> Result<Value, Error> {
    let file = fs::read(&path).with_context(|_| ConfigError::FileOpenFailed)?;
    Ok(serde_json::from_slice(&file).with_context(|_| ConfigError::InvalidJson)?)
}
