use super::*;

#[path = "read_for_each.rs"]
mod for_each;
#[path = "read_lookup.rs"]
mod lookup;
#[path = "read_rows.rs"]
mod rows;

pub(super) fn install(
    object: &mut HashMap<String, JsValue>,
    entries: model::SharedEntries,
    this_value: JsValue,
) {
    lookup::install(object, entries.clone());
    rows::install(object, entries.clone());
    for_each::install(object, entries, this_value);
}
