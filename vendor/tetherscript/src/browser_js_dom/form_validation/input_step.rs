use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    method(obj, handle, "stepUp", 1.0);
    method(obj, handle, "stepDown", -1.0);
}

fn method(
    obj: &Rc<RefCell<HashMap<String, JsValue>>>,
    handle: &DomHandle,
    name: &'static str,
    dir: f64,
) {
    let current = obj.clone();
    let h = handle.clone();
    obj.borrow_mut().insert(
        name.into(),
        native(name, None, move |args| {
            apply(&current, &h, dir, input_number_arg::read(args.first()));
            Ok(JsValue::Undefined)
        }),
    );
}

fn apply(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle, dir: f64, n: f64) {
    let Some(Node::Element(el)) = handle.node() else {
        return;
    };
    if !input_number::numeric(&el) {
        return;
    }
    let count = if n.is_nan() { 1.0 } else { n.trunc() };
    let base = input_number::read(handle, &el);
    let current = if base.is_nan() { 0.0 } else { base };
    handle.set_input_value(input_number_arg::display(current + dir * count * step(&el)));
    input_number::refresh(obj, handle);
}

fn step(el: &Element) -> f64 {
    el.attrs
        .get("step")
        .and_then(|value| value.parse::<f64>().ok())
        .filter(|value| value.is_finite() && *value > 0.0)
        .unwrap_or(1.0)
}
