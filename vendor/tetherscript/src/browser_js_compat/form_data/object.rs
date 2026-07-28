use super::*;

pub(super) fn create(initial: Vec<model::FormEntry>) -> JsValue {
    let entries = Rc::new(RefCell::new(initial));
    let object = Rc::new(RefCell::new(HashMap::new()));
    let this_value = JsValue::Object(object.clone());
    {
        let mut object = object.borrow_mut();
        read::install(&mut object, entries.clone(), this_value.clone());
        write::install(&mut object, entries);
    }
    this_value
}
