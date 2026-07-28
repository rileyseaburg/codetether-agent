use super::*;

pub(super) fn install(window: &mut HashMap<String, JsValue>, root: Rc<RefCell<Node>>) {
    let mut custom = HashMap::new();
    let define_root = root.clone();
    custom.insert(
        "define".into(),
        native("customElements.define", Some(2), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            let value = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let definition = registry::define(name.clone(), value)?;
            upgrade::upgrade_existing(&define_root, &name, &definition)?;
            wait::notify(&name, definition.value)?;
            Ok(JsValue::Undefined)
        }),
    );
    custom.insert(
        "get".into(),
        native("customElements.get", Some(1), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(registry::get(&name)
                .map(|definition| definition.value)
                .unwrap_or(JsValue::Undefined))
        }),
    );
    custom.insert(
        "whenDefined".into(),
        native("customElements.whenDefined", Some(1), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            let ready = registry::get(&name).map(|definition| definition.value);
            Ok(wait::thenable(name, ready))
        }),
    );
    window.insert(
        "customElements".into(),
        JsValue::Object(Rc::new(RefCell::new(custom))),
    );
}
