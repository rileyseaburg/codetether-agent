use super::super::super::*;
use super::create;

pub(super) fn object() -> JsValue {
    let mut obj = HashMap::new();
    install_has_feature(&mut obj);
    install_create_html_document(&mut obj);
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn install_has_feature(obj: &mut HashMap<String, JsValue>) {
    obj.insert(
        "hasFeature".into(),
        native("document.implementation.hasFeature", None, |_| {
            Ok(JsValue::Bool(true))
        }),
    );
}

fn install_create_html_document(obj: &mut HashMap<String, JsValue>) {
    obj.insert(
        "createHTMLDocument".into(),
        native("document.implementation.createHTMLDocument", None, |args| {
            Ok(create::document(args))
        }),
    );
}
