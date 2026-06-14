use super::{process_args, process_spawn};
use tetherscript::value::Value;

pub fn run(progress_id: &str, args: &[Value]) -> Result<Value, String> {
    if !(1..=4).contains(&args.len()) {
        return Err("process_run expects command[, args[, stdin[, timeout_ms]]]".into());
    }
    let command = process_args::string_arg(&args[0], "process_run: command")?;
    let command_args = args.get(1).map_or(Ok(Vec::new()), |v| {
        process_args::list_arg(v, "process_run: args")
    })?;
    let stdin = args.get(2).and_then(stdin_value).transpose()?;
    let timeout = process_args::timeout_arg(args.get(3).unwrap_or(&Value::Nil))?;
    process_spawn::spawn(progress_id, command, command_args, stdin, timeout)
}

fn stdin_value(value: &Value) -> Option<Result<String, String>> {
    if matches!(value, Value::Nil) {
        None
    } else {
        Some(process_args::string_arg(value, "process_run: stdin"))
    }
}
