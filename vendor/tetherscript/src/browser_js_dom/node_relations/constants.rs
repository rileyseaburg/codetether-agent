use super::*;

pub(super) const DISCONNECTED: u16 = 0x01;
pub(super) const PRECEDING: u16 = 0x02;
pub(super) const FOLLOWING: u16 = 0x04;
pub(super) const CONTAINS: u16 = 0x08;
pub(super) const CONTAINED_BY: u16 = 0x10;
pub(super) const IMPLEMENTATION_SPECIFIC: u16 = 0x20;

pub(super) fn install(obj: &mut HashMap<String, JsValue>) {
    for (name, value) in [
        ("DOCUMENT_POSITION_DISCONNECTED", DISCONNECTED),
        ("DOCUMENT_POSITION_PRECEDING", PRECEDING),
        ("DOCUMENT_POSITION_FOLLOWING", FOLLOWING),
        ("DOCUMENT_POSITION_CONTAINS", CONTAINS),
        ("DOCUMENT_POSITION_CONTAINED_BY", CONTAINED_BY),
        (
            "DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC",
            IMPLEMENTATION_SPECIFIC,
        ),
    ] {
        obj.insert(name.into(), JsValue::Number(value as f64));
    }
}
