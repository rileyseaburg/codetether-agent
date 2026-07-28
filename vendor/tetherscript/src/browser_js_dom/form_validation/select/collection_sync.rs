use super::super::*;

const KEYS: &str = "__optionKeys";

pub(super) fn write(obj: &Rc<RefCell<HashMap<String, JsValue>>>, select: &DomHandle) {
    let options = super::handles::all(select);
    let mut obj = obj.borrow_mut();
    clear(&mut obj);
    let mut keys = Vec::new();
    obj.insert("length".into(), JsValue::Number(options.len() as f64));
    for (index, option) in options.into_iter().enumerate() {
        let key = index.to_string();
        obj.insert(
            key.clone(),
            ops::handle_object(option.root.clone(), option.path.clone()),
        );
        keys.push(key);
    }
    obj.insert(KEYS.into(), JsValue::String(keys.join("\n")));
}

fn clear(obj: &mut HashMap<String, JsValue>) {
    let Some(JsValue::String(keys)) = obj.remove(KEYS) else {
        return;
    };
    for key in keys.lines() {
        obj.remove(key);
    }
}
