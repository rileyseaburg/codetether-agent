use super::*;

#[path = "cursor.rs"]
mod cursor;
#[path = "file_object.rs"]
mod file_object;
#[path = "json_number.rs"]
mod json_number;
#[path = "json_string.rs"]
mod json_string;
#[path = "list_each.rs"]
mod list_each;
#[path = "list_object.rs"]
mod list_object;
#[path = "list_rows.rs"]
mod list_rows;
#[path = "meta.rs"]
mod meta;
#[path = "meta_field.rs"]
mod meta_field;

#[derive(Clone, Default)]
pub(super) struct AgentFile {
    pub(super) name: String,
    pub(super) mime_type: String,
    pub(super) size: f64,
    pub(super) last_modified: f64,
}

pub(super) fn install(obj: &mut HashMap<String, JsValue>, node: &Node) {
    let Some(el) = file_input(node) else {
        return;
    };
    let raw = el
        .attrs
        .get("data-agent-files")
        .map(String::as_str)
        .unwrap_or("[]");
    obj.insert("files".into(), list_object::object(meta::parse(raw)));
}

fn file_input(node: &Node) -> Option<&Element> {
    let Node::Element(el) = node else {
        return None;
    };
    let is_file = el
        .attrs
        .get("type")
        .is_some_and(|ty| ty.eq_ignore_ascii_case("file"));
    (el.tag == "input" && is_file).then_some(el)
}
