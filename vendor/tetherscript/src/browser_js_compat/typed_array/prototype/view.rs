use super::*;

pub(super) fn subarray(args: &[JsValue]) -> Result<JsValue, String> {
    let receiver = args
        .first()
        .ok_or_else(|| "Uint8Array.subarray: expected receiver".to_string())?;
    let bytes = bytes::bytes_from_value(receiver);
    Ok(bytes::byte_array(slice(&bytes, args.get(1), args.get(2))))
}

pub(super) fn copy_within(args: &[JsValue]) -> Result<JsValue, String> {
    let target = receiver(args)?;
    let bytes = slice(&bytes::bytes_from_value(&args[0]), args.get(2), args.get(3));
    let start = number::usize(args.get(1), 0);
    for (index, byte) in bytes.into_iter().enumerate() {
        if let Some(slot) = target.borrow_mut().get_mut(start + index) {
            *slot = JsValue::Number(byte as f64);
        }
    }
    Ok(args[0].clone())
}

fn receiver(args: &[JsValue]) -> Result<Rc<RefCell<Vec<JsValue>>>, String> {
    match args.first() {
        Some(JsValue::Array(target)) => Ok(target.clone()),
        _ => Err("Uint8Array.copyWithin: expected typed array receiver".into()),
    }
}

fn slice(bytes: &[u8], start: Option<&JsValue>, end: Option<&JsValue>) -> Vec<u8> {
    let from = number::usize(start, 0);
    let to = number::usize(end, bytes.len());
    bytes[from.min(bytes.len())..to.min(bytes.len())].to_vec()
}
