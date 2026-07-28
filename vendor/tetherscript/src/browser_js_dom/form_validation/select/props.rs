use super::super::*;

pub(super) fn write(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let options = super::handles::all(handle);
    let mut obj = obj.borrow_mut();
    obj.insert("length".into(), JsValue::Number(options.len() as f64));
    obj.insert(
        "selectedIndex".into(),
        JsValue::Number(super::read::selected_index(handle) as f64),
    );
    obj.insert("value".into(), JsValue::String(super::read::value(handle)));
    obj.insert("options".into(), super::collection::create(handle));
}
