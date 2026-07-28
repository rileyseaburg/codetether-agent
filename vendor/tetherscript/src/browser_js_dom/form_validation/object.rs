use super::*;

pub(super) fn validity(value: &check::Validity) -> JsValue {
    let mut obj = HashMap::new();
    for (name, flag) in [
        ("valueMissing", value.value_missing),
        ("typeMismatch", value.type_mismatch),
        ("patternMismatch", false),
        ("tooLong", false),
        ("tooShort", false),
        ("rangeUnderflow", false),
        ("rangeOverflow", false),
        ("stepMismatch", false),
        ("badInput", false),
        ("customError", value.custom_error),
    ] {
        obj.insert(name.into(), JsValue::Bool(flag));
    }
    obj.insert("valid".into(), JsValue::Bool(value.valid()));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}
