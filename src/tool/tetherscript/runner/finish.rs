use anyhow::Result;
use tetherscript::value::Value as TetherScriptValue;

use crate::tool::tetherscript::convert::tetherscript_to_json;

use super::outcome::{self, TetherScriptOutcome};

pub fn finish(
    stdout: String,
    call_result: Result<TetherScriptValue, tetherscript::interp::Unwind>,
) -> Result<TetherScriptOutcome> {
    let (tether_val, success) = match call_result {
        Ok(value) => {
            let success = outcome::is_success(&value);
            (value, success)
        }
        Err(unwind) => (
            TetherScriptValue::Str(std::rc::Rc::new(super::unwind::message(unwind))),
            false,
        ),
    };
    Ok(TetherScriptOutcome {
        output: outcome::output(stdout, &tether_val),
        success,
        value: tetherscript_to_json(&tether_val),
    })
}
