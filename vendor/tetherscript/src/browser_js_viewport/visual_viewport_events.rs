use super::super::*;
use super::event_object;

pub(super) fn install(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    {
        let mut map = object.borrow_mut();
        map.insert("onresize".into(), JsValue::Null);
        map.insert("onscroll".into(), JsValue::Null);
        map.insert("addEventListener".into(), noop("addEventListener"));
        map.insert("removeEventListener".into(), noop("removeEventListener"));
    }
    let target = object.clone();
    object.borrow_mut().insert(
        "dispatchEvent".into(),
        native("visualViewport.dispatchEvent", Some(1), move |args| {
            dispatch(&target, args.first().cloned().unwrap_or(JsValue::Undefined))
        }),
    );
}

fn dispatch(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    event: JsValue,
) -> Result<JsValue, String> {
    let event = event_object::create(object, event);
    let handler = object
        .borrow()
        .get(&format!("on{}", event_object::event_type(&event)))
        .cloned();
    if let Some(handler) = handler.filter(callable) {
        js::call_function_with_this(handler, JsValue::Object(object.clone()), &[event])?;
    }
    Ok(JsValue::Bool(true))
}

fn callable(value: &JsValue) -> bool {
    !matches!(value, JsValue::Undefined | JsValue::Null)
}

fn noop(name: &str) -> JsValue {
    native(&format!("visualViewport.{name}"), None, |_| {
        Ok(JsValue::Undefined)
    })
}
