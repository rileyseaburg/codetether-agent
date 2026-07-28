use super::*;

#[path = "numeric/coerce.rs"]
mod coerce;
#[path = "numeric/source.rs"]
mod source;

pub(super) fn uint32_array(args: &[JsValue]) -> JsValue {
    array(args, 4, coerce::uint32)
}

pub(super) fn uint16_array(args: &[JsValue]) -> JsValue {
    array(args, 2, coerce::uint16)
}

pub(super) fn int32_array(args: &[JsValue]) -> JsValue {
    array(args, 4, coerce::int32)
}

pub(super) fn float32_array(args: &[JsValue]) -> JsValue {
    array(args, 4, coerce::float32)
}

fn array(args: &[JsValue], bytes: usize, coerce: fn(&JsValue) -> JsValue) -> JsValue {
    let source = args.first().unwrap_or(&JsValue::Number(0.0));
    match source {
        JsValue::Number(len) if len.is_finite() && *len > 0.0 => zero_array(*len as usize),
        JsValue::Array(items) => number_array(items.borrow().iter().map(coerce)),
        JsValue::Object(obj) if source::is_array_buffer(&obj.borrow()) => {
            zero_array(source::len(&obj.borrow()) / bytes)
        }
        JsValue::Object(obj) => {
            let len = source::len(&obj.borrow());
            number_array((0..len).map(|index| {
                obj.borrow()
                    .get(&index.to_string())
                    .map(coerce)
                    .unwrap_or(JsValue::Number(0.0))
            }))
        }
        _ => zero_array(0),
    }
}

fn number_array(values: impl IntoIterator<Item = JsValue>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(values.into_iter().collect())))
}

fn zero_array(len: usize) -> JsValue {
    number_array(std::iter::repeat_n(JsValue::Number(0.0), len))
}
