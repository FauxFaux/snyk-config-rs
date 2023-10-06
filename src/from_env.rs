use std::env;

use serde_json::{json, Value};

use super::{ConfigError, Json, Result};

pub fn from_env(prefix: &str) -> Result<Json> {
    let mut map = Json::new();

    for (key, value) in env::vars_os() {
        let key = key.to_string_lossy();
        if !key.starts_with(prefix) {
            continue;
        }

        let key = &key[prefix.len()..];

        let value = value
            .to_str()
            .ok_or_else(|| ConfigError::InvalidEnvEncoding {
                key: key.to_string(),
                value: value.to_string_lossy().into_owned(),
            })?;

        let value = attempt_parse(value);

        merge(&mut map, key, value);
    }

    Ok(map)
}

fn merge(mut map: &mut Json, key: &str, value: Value) {
    let mut path = key.split("__").peekable();
    loop {
        let key = path.next().expect("peeked before");
        if path.peek().is_none() {
            map.insert(key.to_string(), value);
            return;
        }

        // BORROW CHECKER
        let needs_overwrite = match map.entry(key).or_insert_with(|| json!({})) {
            Value::Object(_) => false,
            _ => true,
        };

        if needs_overwrite {
            map.insert(key.to_string(), json!({}));
        }

        map = match map.get_mut(key).expect("just inserted it") {
            Value::Object(map) => map,
            _ => unreachable!("just inserted it"),
        }
    }
}

fn attempt_parse(s: &str) -> Value {
    serde_json::from_str(s).unwrap_or_else(|_| json!(s))
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::Json;

    #[test]
    fn merge_paths_trivial() {
        let mut map = Json::new();
        super::merge(&mut map, "foo", json!("bar"));
        assert_eq!(Value::Object(map), json!({"foo": "bar"}));
    }

    #[test]
    fn merge_paths_single_no_collision() {
        let mut map = Json::new();
        super::merge(&mut map, "foo__bar", json!("baz"));
        assert_eq!(Value::Object(map), json!({"foo": {"bar": "baz"}}));
    }

    #[test]
    fn merge_objects() {
        let mut map = Json::new();
        super::merge(&mut map, "foo", json!(5));
        super::merge(&mut map, "bar", json!(7));
        assert_eq!(Value::Object(map), json!({"foo": 5, "bar": 7}));
    }

    #[test]
    fn merge_paths() {
        let mut map = unwrap_object(&json!({"foo": {"one": 1, "two": 2}}));
        super::merge(&mut map, "foo__two", json!(7));
        assert_eq!(Value::Object(map), json!({"foo": {"one": 1, "two": 7}}));
    }

    #[test]
    fn attempt_parse() {
        assert_eq!(super::attempt_parse("foo"), json!("foo"));
        assert_eq!(super::attempt_parse("[5, 6"), json!("[5, 6"));

        assert_eq!(super::attempt_parse("5"), json!(5));
        assert_eq!(super::attempt_parse("[5, 6]"), json!([5, 6]));
    }

    fn unwrap_object(value: &Value) -> Json {
        match value {
            Value::Object(map) => map.clone(),
            _ => panic!("not an object: {:?}", value),
        }
    }
}
