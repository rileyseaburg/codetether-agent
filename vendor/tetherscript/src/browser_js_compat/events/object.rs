use super::*;

pub(super) fn create(event_class: class::EventClass, args: &[JsValue]) -> JsValue {
    let event_type = args
        .first()
        .map(JsValue::display)
        .unwrap_or_else(|| "event".into());
    let init = args.get(1);
    let object = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut map = object.borrow_mut();
        base::insert_common(&mut map, event_type, init);
        fields::insert(&mut map, event_class, init);
    }
    methods::install(&object, event_class);
    JsValue::Object(object)
}
