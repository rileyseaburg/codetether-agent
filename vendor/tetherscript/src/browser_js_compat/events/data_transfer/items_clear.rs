use super::*;

pub(super) fn install(object: &items::ItemObject, items: &items::SharedItems) {
    let target = object.clone();
    let entries = items.clone();
    object.borrow_mut().insert(
        "clear".into(),
        native("DataTransferItemList.clear", Some(0), move |_| {
            entries.borrow_mut().clear();
            items::sync(&target, &entries);
            Ok(JsValue::Undefined)
        }),
    );
}
