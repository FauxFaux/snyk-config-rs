use serde_json::Value;

use super::Json;

pub fn merge_sources<'a, I: IntoIterator<Item = &'a Value>>(start: Value, rest: I) -> Json {
    let mut current = start;
    for map in rest {
        merge(&mut current, map);
    }
    match current {
        Value::Object(v) => v,
        other => {
            let mut value = Json::new();
            value.insert("value".to_string(), other);
            value
        }
    }
}

// https://github.com/serde-rs/json/issues/377#issuecomment-341490464
fn merge(sink: &mut Value, extra: &Value) {
    match (sink, extra) {
        (&mut Value::Object(ref mut sink), Value::Object(ref extra)) => {
            for (key, value) in extra {
                merge(sink.entry(key.to_string()).or_insert(Value::Null), value);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::merge_sources;

    #[test]
    fn merges() {
        assert_eq!(
            Value::Object(merge_sources(json!({}), &[json!({"one": 2})])),
            json!({"one": 2})
        );
        assert_eq!(
            Value::Object(merge_sources(json!({"one": 2}), &[json!({})])),
            json!({"one": 2})
        );
        assert_eq!(
            Value::Object(merge_sources(json!({"one": 1}), &[json!({"two": 2})])),
            json!({"one": 1, "two": 2})
        );
        assert_eq!(
            Value::Object(merge_sources(
                json!({"one": {"two": 3}}),
                &[json!({"one": {"two": 5}})]
            )),
            json!({"one": {"two": 5}})
        );
    }
}
