use serde_json::Value;

// Google's protocol slots must be arrays, never objects with numeric keys.
pub(super) fn at<'a>(mut value: &'a Value, path: &[usize]) -> Option<&'a Value> {
    for index in path {
        value = value.as_array()?.get(*index)?;
    }
    Some(value)
}
