use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::model::Declaration;
use super::native;
use super::parse_decl;

pub(super) fn object(declarations: &[Declaration]) -> JsValue {
    let mut props = declarations
        .iter()
        .map(|decl| (decl.name.clone(), JsValue::String(decl.value.clone())))
        .collect::<HashMap<_, _>>();
    props.insert(
        "cssText".into(),
        JsValue::String(parse_decl::css_text(declarations)),
    );
    let props = Rc::new(RefCell::new(props));
    let getter_props = props.clone();
    props.borrow_mut().insert(
        "getPropertyValue".into(),
        native(
            "CSSStyleDeclaration.getPropertyValue",
            Some(1),
            move |args| {
                let name = args.first().unwrap_or(&JsValue::Undefined).display();
                Ok(getter_props
                    .borrow()
                    .get(&name.to_ascii_lowercase())
                    .cloned()
                    .unwrap_or_else(|| JsValue::String(String::new())))
            },
        ),
    );
    JsValue::Object(props)
}
