use super::*;

const KEYS: &str = "__formControlKeys";

pub(super) fn write(obj: &Rc<RefCell<HashMap<String, JsValue>>>, form: &DomHandle) -> usize {
    let controls = listed::handles(form);
    let len = controls.len();
    let mut obj = obj.borrow_mut();
    clear(&mut obj);
    let mut keys = Vec::new();
    obj.insert("length".into(), JsValue::Number(len as f64));
    for (index, control) in controls.into_iter().enumerate() {
        let value = ops::handle_object(control.root.clone(), control.path.clone());
        let key = index.to_string();
        obj.insert(key.clone(), value.clone());
        keys.push(key);
        add_names(&mut obj, &mut keys, &control, &value);
    }
    obj.insert(KEYS.into(), JsValue::String(keys.join("\n")));
    len
}

fn add_names(
    obj: &mut HashMap<String, JsValue>,
    keys: &mut Vec<String>,
    control: &DomHandle,
    value: &JsValue,
) {
    for name in names::for_handle(control) {
        if obj.contains_key(&name) {
            continue;
        }
        obj.insert(name.clone(), value.clone());
        keys.push(name);
    }
}

fn clear(obj: &mut HashMap<String, JsValue>) {
    let old = obj.remove(KEYS);
    let Some(JsValue::String(keys)) = old else {
        return;
    };
    for key in keys.lines() {
        obj.remove(key);
    }
}
