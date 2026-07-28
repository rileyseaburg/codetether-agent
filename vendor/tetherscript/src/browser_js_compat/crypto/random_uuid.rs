use super::*;

pub(super) fn install(crypto: &mut HashMap<String, JsValue>, seed: Rc<RefCell<u64>>) {
    crypto.insert(
        "randomUUID".into(),
        native("crypto.randomUUID", Some(0), move |_| {
            Ok(JsValue::String(uuid(&seed)))
        }),
    );
}

fn uuid(seed: &Rc<RefCell<u64>>) -> String {
    let mut bytes = [0; 16];
    {
        let mut state = seed.borrow_mut();
        for byte in &mut bytes {
            *byte = seed::next_byte(&mut state);
        }
    }
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    format!(
        "{}-{}-{}-{}-{}",
        hex(&bytes[0..4]),
        hex(&bytes[4..6]),
        hex(&bytes[6..8]),
        hex(&bytes[8..10]),
        hex(&bytes[10..16])
    )
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::new();
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}
