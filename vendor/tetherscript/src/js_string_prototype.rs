use super::*;

pub(super) fn install() {
    let Some(JsValue::Object(prototype)) = js_prototypes::get("String") else {
        return;
    };
    let mut proto = prototype.borrow_mut();
    proto.insert("indexOf".into(), native("indexOf", Some(2), index_of));
    proto.insert("replace".into(), native("replace", Some(3), replace));
    proto.insert("slice".into(), native("slice", Some(2), slice));
    proto.insert("valueOf".into(), native("valueOf", Some(1), value_of));
}

fn native(
    name: &'static str,
    arity: Option<usize>,
    func: fn(&[JsValue]) -> Result<JsValue, String>,
) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(
        format!("String.prototype.{name}"),
        arity,
        func,
    )))
}

fn index_of(args: &[JsValue]) -> Result<JsValue, String> {
    let text = args[0].display();
    let needle = args[1].display();
    let start = args
        .get(2)
        .map_or(0, |value| value.number().max(0.0) as usize);
    Ok(JsValue::Number(
        text.get(start..)
            .and_then(|tail| tail.find(&needle).map(|index| start + index))
            .map(|index| index as f64)
            .unwrap_or(-1.0),
    ))
}

fn replace(args: &[JsValue]) -> Result<JsValue, String> {
    string_replace(&args[0].display(), &args[1], &args[2])
}

fn value_of(args: &[JsValue]) -> Result<JsValue, String> {
    match &args[0] {
        JsValue::String(text) => Ok(JsValue::String(text.clone())),
        _ => Err("TypeError: String.prototype.valueOf requires String".into()),
    }
}

fn slice(args: &[JsValue]) -> Result<JsValue, String> {
    let text = args[0].display();
    let len = text.chars().count();
    let start = index(args.get(1), len, 0);
    let end = index(args.get(2), len, len);
    Ok(JsValue::String(
        text.chars()
            .skip(start)
            .take(end.saturating_sub(start))
            .collect(),
    ))
}

fn index(value: Option<&JsValue>, len: usize, default: usize) -> usize {
    let Some(value) = value else {
        return default;
    };
    let raw = value.number().trunc() as isize;
    if raw < 0 {
        (len as isize + raw).max(0) as usize
    } else {
        (raw as usize).min(len)
    }
}
