use super::*;
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    let mut scheduling = HashMap::new();
    scheduling.insert(
        "isInputPending".into(),
        native("navigator.scheduling.isInputPending", None, |_| {
            Ok(JsValue::Bool(false))
        }),
    );
    navigator.insert(
        "scheduling".into(),
        JsValue::Object(Rc::new(RefCell::new(scheduling))),
    );
}
