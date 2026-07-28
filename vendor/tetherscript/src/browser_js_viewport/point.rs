use super::*;

pub(super) fn from_args(args: &[JsValue]) -> (i64, i64) {
    (number(args.first()), number(args.get(1)))
}

pub(super) fn scrolled(
    document: &Rc<RefCell<HashMap<String, JsValue>>>,
    x: i64,
    y: i64,
) -> (i64, i64) {
    let Some(JsValue::Object(window)) = document.borrow().get("defaultView").cloned() else {
        return (x, y);
    };
    let window = window.borrow();
    (
        x.saturating_add(number(window.get("scrollX"))),
        y.saturating_add(number(window.get("scrollY"))),
    )
}

fn number(value: Option<&JsValue>) -> i64 {
    value
        .map(JsValue::display)
        .and_then(|raw| raw.parse::<f64>().ok())
        .filter(|value| value.is_finite())
        .unwrap_or(0.0)
        .trunc() as i64
}
