use serde_json::Value;

pub fn get_mut_path<'a>(value: &'a mut Value, spec: &str) -> Option<&'a mut Value> {
    let mut current = value;
    for part in spec.split('.') {
        current = match current {
            Value::Object(map) => map.get_mut(part)?,
            Value::Array(items) => items.get_mut(part.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(current)
}
