use super::*;

pub(super) fn install(object: &items::ItemObject, items: &items::SharedItems) {
    let target = object.clone();
    let entries = items.clone();
    object.borrow_mut().insert(
        "add".into(),
        native("DataTransferItemList.add", None, move |args| {
            let value = args.first().cloned().unwrap_or(JsValue::Undefined);
            let type_name = args.get(1).map(JsValue::display).unwrap_or_default();
            let kind = if args.get(1).is_some() {
                "string"
            } else {
                "file"
            };
            let item = items::item(value, type_name, kind);
            entries.borrow_mut().push(item.clone());
            items::sync(&target, &entries);
            Ok(item)
        }),
    );
}
