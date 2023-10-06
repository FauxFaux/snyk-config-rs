use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde_json::json;

#[test]
fn example() -> Result<()> {
    let dir = tempfile::tempdir()?;
    fs::write(
        join(dir.path(), "config.default.json"),
        r#"{"port": 1337, "host": "localhost"}"#,
    )?;
    fs::write(
        join(dir.path(), "config.local.json"),
        r#"{"host": "0.0.0.0"}"#,
    )?;
    fs::write(
        join(dir.path(), "config.secret.json"),
        r#"{"keys": {"google": "afaf"}}"#,
    )?;
    assert_eq!(
        &snyk_config::Config::for_dir(dir.path())?,
        json!({
            "port": 1337,
            "host": "0.0.0.0",
            "keys": {
                "google": "afaf",
            }
        })
        .as_object()
        .unwrap()
    );
    Ok(())
}

fn join<P: AsRef<Path>, Q: AsRef<Path>>(left: P, right: Q) -> PathBuf {
    let mut left = left.as_ref().to_path_buf();
    left.push(right.as_ref());
    left
}
