use anyhow::Result;
use kiln::plugin::{KilnAuthority, PluginHost};
use kiln::value::{ResultValue, Value as KilnValue};
use serde_json::Value;

use super::convert::{json_to_kiln, kiln_to_json};

#[derive(Debug)]
pub struct KilnOutcome {
    pub output: String,
    pub success: bool,
    pub value: Value,
}

pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
) -> Result<KilnOutcome> {
    let mut host = PluginHost::new();
    host.grant("kiln", KilnAuthority::new());
    let mut plugin = host.load_source(source_name, &source)?;
    let args = args.into_iter().map(json_to_kiln).collect::<Vec<_>>();
    let call = plugin.call(&hook, &args)?;

    let success = !matches!(&call.value, KilnValue::Result(result) if matches!(result.as_ref(), ResultValue::Err(_)));
    let value = kiln_to_json(&call.value);
    let output = format_output(call.stdout, &call.value);

    Ok(KilnOutcome {
        output,
        success,
        value,
    })
}

fn format_output(mut stdout: String, value: &KilnValue) -> String {
    let value_is_empty = matches!(value, KilnValue::Nil)
        || matches!(
            value,
            KilnValue::Result(result) if matches!(result.as_ref(), ResultValue::Ok(KilnValue::Nil))
        );

    if !value_is_empty {
        if !stdout.is_empty() && !stdout.ends_with('\n') {
            stdout.push('\n');
        }
        stdout.push_str(&value.to_string());
    }
    stdout
}
