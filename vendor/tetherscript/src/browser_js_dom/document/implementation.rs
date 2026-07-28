use super::super::*;

#[path = "implementation/create.rs"]
mod create;
#[path = "implementation/doctype.rs"]
mod doctype;
#[path = "implementation/object.rs"]
mod object;

#[cfg(test)]
#[path = "implementation/tests_create.rs"]
mod tests_create;
#[cfg(test)]
#[path = "implementation/tests_doctype.rs"]
mod tests_doctype;
#[cfg(test)]
#[path = "implementation/tests_feature.rs"]
mod tests_feature;
#[cfg(test)]
#[path = "implementation/tests_title_body.rs"]
mod tests_title_body;

pub(super) fn install(obj: &mut HashMap<String, JsValue>) {
    let doc_type = doctype::object();
    obj.insert("doctype".into(), doc_type.clone());
    obj.insert("documentType".into(), doc_type);
    obj.insert("implementation".into(), object::object());
}
