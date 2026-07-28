use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let mut obj = obj.borrow_mut();
    obj.entry("indeterminate".into())
        .or_insert(JsValue::Bool(false));
    obj.insert(
        "__set:indeterminate".into(),
        native("set_indeterminate", Some(1), move |args| {
            Ok(JsValue::Bool(
                args.first().unwrap_or(&JsValue::Undefined).truthy(),
            ))
        }),
    );
}
