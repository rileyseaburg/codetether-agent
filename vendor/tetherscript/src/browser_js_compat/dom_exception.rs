use super::*;

#[path = "dom_exception/codes.rs"]
mod codes;
#[path = "dom_exception/object.rs"]
mod object;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "DOMException".into(),
        native("DOMException", None, move |args| Ok(object::create(args))),
    );
}
