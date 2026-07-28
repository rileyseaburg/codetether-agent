use super::*;
use std::cell::RefCell;
use std::rc::Rc;

#[path = "user_agent_data/brand.rs"]
mod brand;
#[path = "user_agent_data/hints.rs"]
mod hints;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    navigator.insert("userAgentData".into(), object());
}

fn object() -> JsValue {
    let mut object = base();
    object.insert(
        "getHighEntropyValues".into(),
        native(
            "navigator.userAgentData.getHighEntropyValues",
            Some(1),
            |args| Ok(thenable::resolved(values(args.first()))),
        ),
    );
    object.insert(
        "toJSON".into(),
        native("navigator.userAgentData.toJSON", Some(0), |_| {
            Ok(values(None))
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn values(hints: Option<&JsValue>) -> JsValue {
    let mut object = base();
    if let Some(JsValue::Array(hints)) = hints {
        for hint in hints.borrow().iter() {
            hints::insert(&mut object, &hint.display());
        }
    }
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn base() -> HashMap<String, JsValue> {
    HashMap::from([
        ("brands".into(), brand::list()),
        ("mobile".into(), JsValue::Bool(false)),
        ("platform".into(), JsValue::String("TetherScript".into())),
    ])
}
