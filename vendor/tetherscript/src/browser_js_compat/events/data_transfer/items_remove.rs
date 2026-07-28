use super::*;

pub(super) fn install(object: &items::ItemObject, items: &items::SharedItems) {
    let target = object.clone();
    let entries = items.clone();
    object.borrow_mut().insert(
        "remove".into(),
        native("DataTransferItemList.remove", Some(1), move |args| {
            if let Some(index) = items::index_arg(args) {
                let mut entries = entries.borrow_mut();
                if index < entries.len() {
                    entries.remove(index);
                }
            }
            items::sync(&target, &entries);
            Ok(JsValue::Undefined)
        }),
    );
}
