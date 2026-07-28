use super::*;

pub(super) fn install(object: &Rc<RefCell<HashMap<String, JsValue>>>, listeners: Listeners) {
    insert_add(object, listeners.clone());
    insert_remove(object, listeners);
}

fn insert_add(object: &Rc<RefCell<HashMap<String, JsValue>>>, listeners: Listeners) {
    object.borrow_mut().insert(
        "addEventListener".into(),
        native("screen.orientation.addEventListener", None, move |args| {
            if args
                .first()
                .is_some_and(|event| event.display() == "change")
            {
                listeners
                    .borrow_mut()
                    .push(args.get(1).cloned().unwrap_or(JsValue::Undefined));
            }
            Ok(JsValue::Undefined)
        }),
    );
}

fn insert_remove(object: &Rc<RefCell<HashMap<String, JsValue>>>, listeners: Listeners) {
    object.borrow_mut().insert(
        "removeEventListener".into(),
        native(
            "screen.orientation.removeEventListener",
            None,
            move |args| {
                let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
                listeners.borrow_mut().retain(|item| item != &listener);
                Ok(JsValue::Undefined)
            },
        ),
    );
}
