use super::*;

pub(super) fn result(args: &[JsValue], target: &model::Parts) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("inputs".into(), inputs(args));
    obj.insert("protocol".into(), component(&target.protocol));
    obj.insert("hostname".into(), component(&target.hostname));
    obj.insert("pathname".into(), component(&target.pathname));
    obj.insert("search".into(), component(&target.search));
    obj.insert("hash".into(), component(&target.hash));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn inputs(args: &[JsValue]) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(
        args.iter().take(2).cloned().collect(),
    )))
}

fn component(input: &str) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("input".into(), JsValue::String(input.into()));
    obj.insert(
        "groups".into(),
        JsValue::Object(Rc::new(RefCell::new(HashMap::new()))),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}
