use tetherscript::value::Value;

use super::{computer_scopes, computer_value};

/// Build a v1 bridge payload and return its required scope.
pub fn prepare(method: &str, args: &[Value]) -> Result<(Value, &'static str), String> {
    let action = action_name(method, args)?;
    let scope = computer_scopes::for_action(&action)
        .ok_or_else(|| format!("computer.{method}: unsupported action `{action}`"))?;
    Ok((payload(&action, method, args)?, scope))
}

fn action_name(method: &str, args: &[Value]) -> Result<String, String> {
    if method != "raw" {
        return Ok(method.to_string());
    }
    match args.first() {
        Some(Value::Str(action)) => Ok((**action).clone()),
        _ => Err("computer.raw: first argument must be action string".into()),
    }
}

fn payload(action: &str, method: &str, args: &[Value]) -> Result<Value, String> {
    let params = if method == "raw" {
        args.get(1)
    } else {
        args.first()
    };
    let mut entries = vec![("action".into(), computer_value::str_value(action))];
    match params {
        None | Some(Value::Nil) => {}
        Some(Value::Map(map)) => entries.extend(
            map.borrow()
                .iter()
                .filter(|(k, _)| *k != "action")
                .map(|(k, v)| (k.clone(), v.clone())),
        ),
        Some(other) => {
            return Err(format!(
                "computer.{method}: params must be map, got {}",
                other.type_name()
            ));
        }
    }
    Ok(computer_value::map(entries))
}
