//! Deterministic popup proxy returned by `window.open`.

use super::*;

pub(super) fn install(window: &WindowObject, opener: JsValue) {
    window.borrow_mut().insert(
        "open".into(),
        native("window.open", None, move |args| {
            Ok(make_popup(args, opener.clone()))
        }),
    );
}

fn make_popup(args: &[JsValue], opener: JsValue) -> JsValue {
    let url = arg_string(args, 0, "about:blank");
    let name = arg_string(args, 1, "");
    let features = arg_string(args, 2, "");
    let popup = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut fields = popup.borrow_mut();
        fields.insert("closed".into(), JsValue::Bool(false));
        fields.insert("__focused".into(), JsValue::Bool(true));
        fields.insert("opener".into(), opener);
        fields.insert("name".into(), JsValue::String(name.clone()));
        fields.insert("target".into(), JsValue::String(name));
        fields.insert("url".into(), JsValue::String(url.clone()));
        fields.insert("features".into(), JsValue::String(features));
        fields.insert("location".into(), location(&url));
        methods::install_popup(&popup, &mut fields);
    }
    JsValue::Object(popup)
}

fn arg_string(args: &[JsValue], index: usize, default: &str) -> String {
    match args.get(index) {
        None | Some(JsValue::Undefined) => default.into(),
        Some(value) => value.display(),
    }
}

fn location(url: &str) -> JsValue {
    let mut location = HashMap::new();
    location.insert("href".into(), JsValue::String(url.into()));
    JsValue::Object(Rc::new(RefCell::new(location)))
}
