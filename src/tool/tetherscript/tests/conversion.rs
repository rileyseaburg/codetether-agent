use std::{cell::RefCell, rc::Rc};

use serde_json::{Number, Value};
use tetherscript::value::Value as TetherScriptValue;
use tetherscript::value::resource::OwnedResource;

use super::super::convert::{json_to_tetherscript, tetherscript_to_json};

#[test]
fn large_unsigned_numbers_become_strings() {
    let value = json_to_tetherscript(Value::Number(Number::from(u64::MAX)));

    match value {
        TetherScriptValue::Str(value) => assert_eq!(&*value, &u64::MAX.to_string()),
        other => panic!("expected string for oversized integer, got {other:?}"),
    }
}

#[test]
fn resources_become_descriptive_strings() {
    let resource = TetherScriptValue::Resource(Rc::new(RefCell::new(OwnedResource::task())));

    assert_eq!(
        tetherscript_to_json(&resource),
        Value::String("<task resource (open)>".to_string())
    );
}
