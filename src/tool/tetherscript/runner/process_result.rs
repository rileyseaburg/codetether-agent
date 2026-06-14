use super::process_types::{PipeOutput, map, string};
use std::collections::HashMap;
use std::process::ExitStatus;
use tetherscript::value::Value;

pub fn value(status: ExitStatus, timed_out: bool, stdout: PipeOutput, stderr: PipeOutput) -> Value {
    let mut fields = HashMap::new();
    fields.insert(
        "success".into(),
        Value::Bool(status.success() && !timed_out),
    );
    fields.insert("timed_out".into(), Value::Bool(timed_out));
    fields.insert("code".into(), code(status));
    fields.insert("stdout".into(), string(stdout.text));
    fields.insert("stderr".into(), string(stderr.text));
    fields.insert("stdout_truncated".into(), Value::Bool(stdout.truncated));
    fields.insert("stderr_truncated".into(), Value::Bool(stderr.truncated));
    map(fields)
}

fn code(status: ExitStatus) -> Value {
    status
        .code()
        .map(|code| Value::Int(code as i64))
        .unwrap_or(Value::Nil)
}
