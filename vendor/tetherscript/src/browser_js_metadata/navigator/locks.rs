use super::*;
use crate::js;
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    let held = Rc::new(RefCell::new(Vec::new()));
    let mut locks = HashMap::new();
    locks.insert(
        "request".into(),
        native("navigator.locks.request", None, move |args| {
            request(args, held.clone())
        }),
    );
    navigator.insert(
        "locks".into(),
        JsValue::Object(Rc::new(RefCell::new(locks))),
    );
}

fn request(args: &[JsValue], held: Rc<RefCell<Vec<String>>>) -> Result<JsValue, String> {
    let name = args.first().unwrap_or(&JsValue::Undefined).display();
    if held.borrow().iter().any(|held| held == &name) {
        return Err(format!(
            "navigator.locks.request: lock '{name}' is already held"
        ));
    }
    let callback = args.get(1).cloned().unwrap_or(JsValue::Undefined);
    held.borrow_mut().push(name.clone());
    let result =
        js::call_function_with_this(callback, JsValue::Undefined, &[lock_object(name.clone())]);
    release(&held, &name);
    result.map(thenable::resolved)
}

fn release(held: &Rc<RefCell<Vec<String>>>, name: &str) {
    held.borrow_mut().retain(|held| held != name);
}

fn lock_object(name: String) -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([
        ("name".into(), JsValue::String(name)),
        ("mode".into(), JsValue::String("exclusive".into())),
    ]))))
}
