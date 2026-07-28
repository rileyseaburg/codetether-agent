use crate::js::JsValue;

pub(super) const DEFAULT: u64 = 0x8f_31_ba_4c_2d_19_70_e5;

pub(super) fn next_byte(seed: &mut u64) -> u8 {
    *seed = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    (*seed >> 32) as u8
}

pub(super) fn value(value: Option<&JsValue>) -> u64 {
    match value {
        Some(JsValue::Number(value)) if value.is_finite() => *value as u64,
        Some(JsValue::String(value)) => hash(value.as_bytes()),
        Some(value) => hash(value.display().as_bytes()),
        None => DEFAULT,
    }
}

fn hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(DEFAULT, |hash, byte| {
        (hash ^ *byte as u64).wrapping_mul(0x100_0000_01b3)
    })
}
