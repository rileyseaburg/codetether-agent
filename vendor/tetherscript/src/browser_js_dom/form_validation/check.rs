use super::*;

#[derive(Clone, Default)]
pub(super) struct Validity {
    pub(super) custom_error: bool,
    pub(super) type_mismatch: bool,
    pub(super) value_missing: bool,
}

impl Validity {
    pub(super) fn valid(&self) -> bool {
        !self.custom_error && !self.type_mismatch && !self.value_missing
    }
}

pub(super) fn validity(handle: &DomHandle) -> Validity {
    let Some(Node::Element(el)) = handle.node() else {
        return Validity::default();
    };
    if !controls::will_validate(&el) {
        return Validity::default();
    }
    let value = values::control(handle);
    let input_type = controls::input_type(&el);
    Validity {
        custom_error: custom::get(handle).is_some(),
        type_mismatch: el.tag == "input" && types::mismatch(&input_type, &value),
        value_missing: value_missing(&el, &input_type, &value),
    }
}

fn value_missing(el: &Element, input_type: &str, value: &str) -> bool {
    if !el.attrs.contains_key("required") {
        return false;
    }
    if el.tag == "input" && matches!(input_type, "checkbox" | "radio") {
        return !el.attrs.contains_key("checked");
    }
    value.is_empty()
}
