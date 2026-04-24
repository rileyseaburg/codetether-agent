use serde_json::{Number, Value};
use tetherscript::value::Value as TetherScriptValue;

use super::super::convert::json_to_tetherscript;

#[test]
fn large_unsigned_numbers_become_strings() {
    let value = json_to_tetherscript(Value::Number(Number::from(u64::MAX)));

    match value {
        TetherScriptValue::Str(value) => assert_eq!(&*value, &u64::MAX.to_string()),
        other => panic!("expected string for oversized integer, got {other:?}"),
    }
}
