use super::*;

pub(super) fn text(handle: &DomHandle) -> String {
    if let Some(message) = custom::get(handle) {
        return message;
    }
    let validity = check::validity(handle);
    if validity.value_missing {
        return "Please fill out this field.".into();
    }
    if validity.type_mismatch {
        return type_message(handle);
    }
    String::new()
}

fn type_message(handle: &DomHandle) -> String {
    match handle.node() {
        Some(Node::Element(el)) if controls::input_type(&el) == "email" => {
            "Please enter a valid email address.".into()
        }
        Some(Node::Element(el)) if controls::input_type(&el) == "url" => {
            "Please enter a valid URL.".into()
        }
        _ => "Please enter a valid value.".into(),
    }
}
