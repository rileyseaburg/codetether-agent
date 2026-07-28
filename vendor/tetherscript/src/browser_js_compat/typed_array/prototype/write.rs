use super::*;

pub(super) fn set(args: &[JsValue]) -> Result<JsValue, String> {
    let target = receiver(args, "set")?;
    let source = args
        .get(1)
        .ok_or_else(|| "Uint8Array.set: expected source".to_string())?;
    let offset = number::usize(args.get(2), 0);
    for (index, byte) in bytes::bytes_from_value(source).into_iter().enumerate() {
        if let Some(slot) = target.borrow_mut().get_mut(offset + index) {
            *slot = JsValue::Number(byte as f64);
        }
    }
    Ok(JsValue::Undefined)
}

pub(super) fn fill(args: &[JsValue]) -> Result<JsValue, String> {
    let target = receiver(args, "fill")?;
    let value = JsValue::Number(number::byte(args.get(1)) as f64);
    for slot in target.borrow_mut().iter_mut() {
        *slot = value.clone();
    }
    Ok(args[0].clone())
}

fn receiver(args: &[JsValue], method: &str) -> Result<Rc<RefCell<Vec<JsValue>>>, String> {
    match args.first() {
        Some(JsValue::Array(target)) => Ok(target.clone()),
        _ => Err(format!(
            "Uint8Array.{method}: expected typed array receiver"
        )),
    }
}
