use super::*;

#[path = "clipboard_item/object.rs"]
mod object;
#[path = "clipboard_item/reject.rs"]
mod reject;
#[path = "clipboard_item/value.rs"]
mod value;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "ClipboardItem".into(),
        native("ClipboardItem", Some(1), object::construct),
    );
}
