use super::*;

pub(super) fn install(
    object: &mut HashMap<String, JsValue>,
    strings: model::SharedStrings,
    types: model::SharedArray,
) {
    data_set::install(object, strings.clone(), types.clone());
    data_get::install(object, strings.clone());
    data_clear::install(object, strings, types);
}
