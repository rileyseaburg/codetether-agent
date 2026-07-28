use super::*;

const CONSTANTS: [(&str, f64); 12] = [
    ("ELEMENT_NODE", 1.0),
    ("ATTRIBUTE_NODE", 2.0),
    ("TEXT_NODE", 3.0),
    ("DOCUMENT_NODE", 9.0),
    ("DOCUMENT_TYPE_NODE", 10.0),
    ("DOCUMENT_FRAGMENT_NODE", 11.0),
    ("DOCUMENT_POSITION_DISCONNECTED", 1.0),
    ("DOCUMENT_POSITION_PRECEDING", 2.0),
    ("DOCUMENT_POSITION_FOLLOWING", 4.0),
    ("DOCUMENT_POSITION_CONTAINS", 8.0),
    ("DOCUMENT_POSITION_CONTAINED_BY", 16.0),
    ("DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC", 32.0),
];

pub(super) fn create() -> JsValue {
    let mut ctor = NativeFunction::new("Node", None, |_| {
        Err("TypeError: Node constructor is not supported".into())
    });
    for (name, value) in CONSTANTS {
        ctor = ctor.with_property(name, JsValue::Number(value));
    }
    JsValue::Native(Rc::new(ctor))
}
