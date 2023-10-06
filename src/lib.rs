use std::env;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use thiserror::Error;

mod from_env;
mod from_file;
mod merge_sources;

use merge_sources::merge_sources;

pub type Json = Map<String, Value>;
pub type Result<T> = std::result::Result<T, ConfigError>;

pub struct Config {
    pub prefix: String,
    pub dir: PathBuf,
    pub secrets_file: PathBuf,
    pub service_env: OsString,
    _use_default_default: (),
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: "CONF_".to_string(),
            dir: PathBuf::new(),
            secrets_file: PathBuf::from(
                env::var_os("CONFIG_SECRET_FILE")
                    .unwrap_or_else(|| OsString::from("config.secret.json")),
            ),
            service_env: env::var_os("SERVICE_ENV").unwrap_or_else(|| OsString::from("local")),
            _use_default_default: (),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid utf-8 in {key:?}, got roughly {value:?}")]
    InvalidEnvEncoding { key: String, value: String },

    #[error("locating file failed: {path:?} (in {cwd:?})")]
    ResolvePath {
        source: io::Error,
        path: PathBuf,
        cwd: io::Result<PathBuf>,
    },

    #[error("open {path:?} failed")]
    FileOpenFailed { source: io::Error, path: PathBuf },

    #[error("invalid json in  {path:?}")]
    InvalidJson {
        source: serde_json::Error,
        path: PathBuf,
    },
}

impl Config {
    pub fn for_prefix<S: ToString>(prefix: S) -> Result<Json> {
        Config {
            prefix: prefix.to_string(),
            ..Default::default()
        }
        .load()
    }

    pub fn for_dir<P: AsRef<Path>>(dir: P) -> Result<Json> {
        Config {
            dir: dir.as_ref().to_path_buf(),
            secrets_file: join(
                dir.as_ref().to_path_buf(),
                &OsString::from("config.secret.json"),
            ),
            ..Default::default()
        }
        .load()
    }

    pub fn load(self) -> Result<Json> {
        let default = from_file::load(join(
            self.dir.to_path_buf(),
            &OsString::from("config.default.json"),
        ))?;
        let service_env = from_file::load(join(self.dir, &env_file(&self.service_env)))?;
        let secret = from_file::load(self.secrets_file)?;
        let from_env = from_env::from_env(&self.prefix)?;
        Ok(merge_sources(
            default,
            &[service_env, secret, Value::Object(from_env)],
        ))
    }
}

fn join(mut root: PathBuf, extra: &OsString) -> PathBuf {
    root.push(extra);
    root
}

fn env_file(env: &OsString) -> OsString {
    let mut file = OsString::from("config.");
    file.push(env);
    file.push(".json");
    file
}
