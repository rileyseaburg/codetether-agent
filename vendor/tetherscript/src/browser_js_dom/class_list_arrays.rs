use super::*;

pub(super) fn keys(tokens: &[String]) -> JsValue {
    array(
        (0..tokens.len())
            .map(|index| JsValue::Number(index as f64))
            .collect(),
    )
}

pub(super) fn values(tokens: &[String]) -> JsValue {
    array(tokens.iter().cloned().map(JsValue::String).collect())
}

pub(super) fn entries(tokens: &[String]) -> JsValue {
    array(
        tokens
            .iter()
            .enumerate()
            .map(|(index, token)| {
                array(vec![
                    JsValue::Number(index as f64),
                    JsValue::String(token.clone()),
                ])
            })
            .collect(),
    )
}

fn array(items: Vec<JsValue>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(items)))
}
