//! PerformanceObserver object methods.

use super::*;

pub(super) fn create(callback: JsValue, _timers: Rc<RefCell<TimerQueue>>) -> JsValue {
    let map = Rc::new(RefCell::new(HashMap::new()));
    let object = JsValue::Object(map.clone());
    let id = store::insert(callback, object.clone());
    map.borrow_mut()
        .insert("observe".into(), observe_method(id));
    map.borrow_mut()
        .insert("disconnect".into(), disconnect_method(id));
    map.borrow_mut()
        .insert("takeRecords".into(), take_records_method(id));
    object
}

fn observe_method(id: u64) -> JsValue {
    native("PerformanceObserver.observe", Some(1), move |args| {
        store::observe(id, observed_types(args.first()));
        Ok(JsValue::Undefined)
    })
}

fn disconnect_method(id: u64) -> JsValue {
    native("PerformanceObserver.disconnect", Some(0), move |_| {
        store::disconnect(id);
        Ok(JsValue::Undefined)
    })
}

fn take_records_method(id: u64) -> JsValue {
    native("PerformanceObserver.takeRecords", Some(0), move |_| {
        Ok(entry::array(store::take(id)))
    })
}

fn observed_types(value: Option<&JsValue>) -> Vec<String> {
    let Some(JsValue::Object(options)) = value else {
        return vec!["mark".into(), "measure".into()];
    };
    if let Some(JsValue::Array(types)) = options.borrow().get("entryTypes") {
        return types.borrow().iter().map(JsValue::display).collect();
    }
    options
        .borrow()
        .get("type")
        .map(|value| vec![value.display()])
        .unwrap_or_else(|| vec!["mark".into(), "measure".into()])
}
