use crate::js::JsValue;

pub(super) fn insert(value: Option<&JsValue>, len: usize) -> Result<usize, String> {
    let Some(value) = value else {
        return Ok(len);
    };
    let index = parse(value).ok_or_else(|| "insertRule: invalid index".to_string())?;
    if index > len {
        return Err(format!(
            "insertRule: index {index} exceeds rule count {len}"
        ));
    }
    Ok(index)
}

pub(super) fn delete(value: &JsValue, len: usize) -> Result<usize, String> {
    let index = parse(value).ok_or_else(|| "deleteRule: invalid index".to_string())?;
    if index >= len {
        return Err(format!(
            "deleteRule: index {index} exceeds rule count {len}"
        ));
    }
    Ok(index)
}

fn parse(value: &JsValue) -> Option<usize> {
    match value {
        JsValue::Number(number) if number.is_finite() && *number >= 0.0 => {
            Some(number.trunc() as usize)
        }
        other => other.display().parse().ok(),
    }
}
