use super::super::*;

#[path = "namespace/attr.rs"]
mod attr;
#[path = "namespace/element.rs"]
mod element;
#[path = "namespace/name.rs"]
mod name;

#[cfg(test)]
#[path = "namespace/tests.rs"]
mod tests;

pub(super) fn install(obj: &mut HashMap<String, JsValue>) {
    element::install(obj);
    attr::install(obj);
}
