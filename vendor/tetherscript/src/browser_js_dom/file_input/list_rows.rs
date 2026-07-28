use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, values: Vec<JsValue>) {
    add(obj, "keys", values.clone(), keys);
    add(obj, "values", values.clone(), values_array);
    add(obj, "entries", values, entries);
}

fn add(
    obj: &mut HashMap<String, JsValue>,
    method: &'static str,
    values: Vec<JsValue>,
    build: fn(&[JsValue]) -> JsValue,
) {
    let native_name = format!("FileList.{method}");
    obj.insert(
        method.into(),
        native(&native_name, Some(0), move |_| Ok(build(&values))),
    );
}

fn keys(values: &[JsValue]) -> JsValue {
    array(
        (0..values.len())
            .map(|index| JsValue::Number(index as f64))
            .collect(),
    )
}

fn values_array(values: &[JsValue]) -> JsValue {
    array(values.to_vec())
}

fn entries(values: &[JsValue]) -> JsValue {
    array(
        values
            .iter()
            .enumerate()
            .map(|(index, value)| array(vec![JsValue::Number(index as f64), value.clone()]))
            .collect(),
    )
}

fn array(items: Vec<JsValue>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(items)))
}
