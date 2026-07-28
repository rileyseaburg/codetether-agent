use super::field::Field;
use super::state::DateState;
use super::*;

#[path = "object/fields.rs"]
mod fields;

pub(super) fn new(ms: f64) -> JsValue {
    let date = Rc::new(RefCell::new(DateState::new(ms)));
    let mut obj = HashMap::new();
    obj.insert(
        "getTime".into(),
        methods::value("Date.getTime", date.clone()),
    );
    obj.insert(
        "valueOf".into(),
        methods::value("Date.valueOf", date.clone()),
    );
    install_getters(&mut obj, &date);
    install_setters(&mut obj, &date);
    obj.insert("getTimezoneOffset".into(), methods::timezone_offset());
    obj.insert(
        "toISOString".into(),
        methods::string("Date.toISOString", date.clone()),
    );
    obj.insert("toString".into(), methods::string("Date.toString", date));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn getter(date: &Rc<RefCell<DateState>>, name: &'static str, field: Field) -> JsValue {
    methods::getter(format!("Date.{name}"), date.clone(), field)
}

fn setter(date: &Rc<RefCell<DateState>>, name: &'static str, field: Field) -> JsValue {
    methods::setter(format!("Date.{name}"), date.clone(), field)
}

fn install_getters(obj: &mut HashMap<String, JsValue>, date: &Rc<RefCell<DateState>>) {
    for (name, field) in fields::getters() {
        obj.insert(name.into(), getter(date, name, field));
    }
}

fn install_setters(obj: &mut HashMap<String, JsValue>, date: &Rc<RefCell<DateState>>) {
    for (name, field) in fields::setters() {
        obj.insert(name.into(), setter(date, name, field));
    }
}
