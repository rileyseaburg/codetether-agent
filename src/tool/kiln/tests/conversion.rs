use kiln::value::Value as KilnValue;
use serde_json::{Number, Value};

use super::super::convert::json_to_kiln;

#[test]
fn large_unsigned_numbers_become_strings() {
    let value = json_to_kiln(Value::Number(Number::from(u64::MAX)));

    match value {
        KilnValue::Str(value) => assert_eq!(&*value, &u64::MAX.to_string()),
        other => panic!("expected string for oversized integer, got {other:?}"),
    }
}
