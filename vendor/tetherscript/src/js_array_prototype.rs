use super::*;

type ArrayMethod = fn(Rc<RefCell<Vec<JsValue>>>, &[JsValue]) -> Result<JsValue, String>;

pub(super) fn install(prototype: &mut HashMap<String, JsValue>) {
    for (name, method) in [
        ("forEach", array_for_each as ArrayMethod),
        ("map", array_map as ArrayMethod),
        ("filter", array_filter as ArrayMethod),
        ("find", array_find as ArrayMethod),
        ("findIndex", array_find_index as ArrayMethod),
        ("some", array_some as ArrayMethod),
        ("every", array_every as ArrayMethod),
        ("reduce", array_reduce as ArrayMethod),
        ("reduceRight", array_reduce_right as ArrayMethod),
    ] {
        insert(prototype, name, method);
    }
}

fn insert(prototype: &mut HashMap<String, JsValue>, name: &'static str, method: ArrayMethod) {
    prototype.insert(
        name.into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            format!("Array.prototype.{name}"),
            None,
            move |args| method(empty_array(), args),
        ))),
    );
}

fn empty_array() -> Rc<RefCell<Vec<JsValue>>> {
    Rc::new(RefCell::new(Vec::new()))
}
