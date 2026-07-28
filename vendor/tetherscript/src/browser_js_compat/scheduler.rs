use super::super::*;

#[cfg(test)]
#[path = "scheduler_tests.rs"]
mod tests;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert("scheduler".into(), object());
}

fn object() -> JsValue {
    let mut object = HashMap::new();
    object.insert("postTask".into(), post_task());
    object.insert("yield".into(), yield_task());
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn post_task() -> JsValue {
    native("scheduler.postTask", None, |args| {
        let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
        if matches!(callback, JsValue::Undefined | JsValue::Null) {
            return Ok(promise::api::resolved(JsValue::Undefined));
        }
        Ok(
            match js::call_function_with_this(callback, JsValue::Undefined, &[]) {
                Ok(value) => promise::api::resolved(value),
                Err(error) => promise::api::rejected(JsValue::String(error)),
            },
        )
    })
}

fn yield_task() -> JsValue {
    native("scheduler.yield", None, |_| {
        Ok(promise::api::resolved(JsValue::Undefined))
    })
}
