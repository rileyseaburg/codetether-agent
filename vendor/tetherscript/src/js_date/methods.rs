use super::field::{self, Field};
use super::state::DateState;
use super::*;

type SharedDate = Rc<RefCell<DateState>>;

pub(super) fn value(name: impl Into<String>, date: SharedDate) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, Some(0), move |_| {
        Ok(JsValue::Number(date.borrow().ms()))
    })))
}

pub(super) fn getter(name: impl Into<String>, date: SharedDate, part: Field) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, Some(0), move |_| {
        Ok(JsValue::Number(
            field::read(part, date.borrow().parts()) as f64
        ))
    })))
}

pub(super) fn setter(name: impl Into<String>, date: SharedDate, part: Field) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, Some(1), move |args| {
        let value = args.first().unwrap_or(&JsValue::Undefined).number() as i64;
        let ms = date
            .borrow_mut()
            .set(|parts| field::write(part, parts, value));
        Ok(JsValue::Number(ms))
    })))
}

pub(super) fn timezone_offset() -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(
        "Date.getTimezoneOffset",
        Some(0),
        |_| Ok(JsValue::Number(0.0)),
    )))
}

pub(super) fn string(name: impl Into<String>, date: SharedDate) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, Some(0), move |_| {
        Ok(JsValue::String(date.borrow().parts().iso()))
    })))
}
