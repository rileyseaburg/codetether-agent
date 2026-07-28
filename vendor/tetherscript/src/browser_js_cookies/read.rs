use super::*;

pub(super) fn get() -> JsValue {
    native("CookieStore.get", Some(1), |args| {
        let value = options::name(args.first()).and_then(find);
        Ok(thenable::fulfilled(value.unwrap_or(JsValue::Undefined)))
    })
}

pub(super) fn get_all() -> JsValue {
    native("CookieStore.getAll", None, |args| {
        Ok(thenable::fulfilled(record::array(filter(args.first()))))
    })
}

fn find(name: String) -> Option<JsValue> {
    state::visible_pairs()
        .into_iter()
        .find(|(candidate, _)| candidate == &name)
        .map(|(name, value)| record::cookie(name, value))
}

fn filter(value: Option<&JsValue>) -> Vec<(String, String)> {
    let pairs = state::visible_pairs();
    let Some(name) = options::name(value) else {
        return pairs;
    };
    pairs
        .into_iter()
        .filter(|(candidate, _)| candidate == &name)
        .collect()
}
