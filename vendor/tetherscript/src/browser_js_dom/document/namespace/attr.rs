use super::super::super::*;
use super::name;

pub(super) fn install(obj: &mut HashMap<String, JsValue>) {
    obj.insert(
        "createAttribute".into(),
        native("document.createAttribute", Some(1), |args| {
            Ok(create(None, args.first()))
        }),
    );
    obj.insert(
        "createAttributeNS".into(),
        native("document.createAttributeNS", Some(2), |args| {
            Ok(create(args.first(), args.get(1)))
        }),
    );
}

fn create(namespace_arg: Option<&JsValue>, name_arg: Option<&JsValue>) -> JsValue {
    let namespace = namespace_arg.map_or(JsValue::Null, |value| name::namespace_value(Some(value)));
    let qualified = name::parse(name_arg.unwrap_or(&JsValue::Undefined));
    let prefix = name::prefix_value(&qualified);
    let mut obj = HashMap::new();
    obj.insert("name".into(), JsValue::String(qualified.name.clone()));
    obj.insert("nodeName".into(), JsValue::String(qualified.name));
    obj.insert("nodeType".into(), JsValue::Number(2.0));
    obj.insert("value".into(), JsValue::String(String::new()));
    obj.insert("nodeValue".into(), JsValue::String(String::new()));
    obj.insert("specified".into(), JsValue::Bool(true));
    obj.insert("namespaceURI".into(), namespace);
    obj.insert("localName".into(), JsValue::String(qualified.local));
    obj.insert("prefix".into(), prefix);
    JsValue::Object(Rc::new(RefCell::new(obj)))
}
