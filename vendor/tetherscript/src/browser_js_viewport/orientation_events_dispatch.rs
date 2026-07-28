use super::super::event;
use super::*;

pub(super) fn install(object: &Rc<RefCell<HashMap<String, JsValue>>>, listeners: Listeners) {
    let target = object.clone();
    object.borrow_mut().insert(
        "dispatchEvent".into(),
        native("screen.orientation.dispatchEvent", Some(1), move |args| {
            dispatch(
                &target,
                &listeners,
                args.first().cloned().unwrap_or(JsValue::Undefined),
            )
        }),
    );
}

fn dispatch(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &Listeners,
    raw: JsValue,
) -> Result<JsValue, String> {
    let event = event::create(object, raw);
    if event::event_type(&event) == "change" {
        let this_value = JsValue::Object(object.clone());
        for listener in listeners.borrow().clone() {
            js::call_function_with_this(
                listener,
                this_value.clone(),
                std::slice::from_ref(&event),
            )?;
        }
        if let Some(handler) = object.borrow().get("onchange").cloned().filter(callable) {
            js::call_function_with_this(handler, this_value, std::slice::from_ref(&event))?;
        }
    }
    Ok(JsValue::Bool(true))
}

fn callable(value: &JsValue) -> bool {
    !matches!(value, JsValue::Undefined | JsValue::Null)
}
