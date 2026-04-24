use anyhow::Result;
use serde_json::Value;
use tetherscript::plugin::{PluginHost, TetherScriptAuthority};
use tetherscript::value::{ResultValue, Value as TetherScriptValue};

use super::convert::{json_to_tetherscript, tetherscript_to_json};

#[derive(Debug)]
pub struct TetherScriptOutcome {
    pub output: String,
    pub success: bool,
    pub value: Value,
}

pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
) -> Result<TetherScriptOutcome> {
    let mut host = PluginHost::new();
    host.grant("tetherscript", TetherScriptAuthority::new());
    let mut plugin = host.load_source(source_name, &source)?;
    let args = args
        .into_iter()
        .map(json_to_tetherscript)
        .collect::<Vec<_>>();
    let call = plugin.call(&hook, &args)?;

    let success = !matches!(&call.value, TetherScriptValue::Result(result) if matches!(result.as_ref(), ResultValue::Err(_)));
    let value = tetherscript_to_json(&call.value);
    let output = format_output(call.stdout, &call.value);

    Ok(TetherScriptOutcome {
        output,
        success,
        value,
    })
}

fn format_output(mut stdout: String, value: &TetherScriptValue) -> String {
    let value_is_empty = matches!(value, TetherScriptValue::Nil)
        || matches!(
            value,
            TetherScriptValue::Result(result) if matches!(result.as_ref(), ResultValue::Ok(TetherScriptValue::Nil))
        );

    if !value_is_empty {
        if !stdout.is_empty() && !stdout.ends_with('\n') {
            stdout.push('\n');
        }
        stdout.push_str(&value.to_string());
    }
    stdout
}
