use crate::value::Value;

pub(super) fn apply(value: Option<&Value>, path_prefix: &mut Option<String>) -> Result<(), String> {
    let Some(value) = value else {
        return Ok(());
    };
    let requested = match value {
        Value::Str(prefix) => (**prefix).clone(),
        _ => return Err("http.narrow: `path_prefix` must be a string".into()),
    };
    if let Some(current) = path_prefix {
        if !requested.starts_with(current.as_str()) {
            return Err(format!(
                "http.narrow: prefix {} does not extend current prefix {}",
                requested, current
            ));
        }
    }
    *path_prefix = Some(requested);
    Ok(())
}
