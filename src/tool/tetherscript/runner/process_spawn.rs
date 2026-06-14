use super::{process_io, process_output, process_result, process_types::PipeOutput, process_wait};
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::thread;
use tetherscript::value::Value;

pub fn spawn(
    id: &str,
    command: String,
    args: Vec<String>,
    stdin: Option<String>,
    timeout: u64,
) -> Result<Value, String> {
    let mut child = child(command, args, stdin.is_some())?;
    let out = child.stdout.take().ok_or("process_run: stdout missing")?;
    let err = child.stderr.take().ok_or("process_run: stderr missing")?;
    let out_reader = thread::spawn(reader(out, id.to_string(), "stdout"));
    let err_reader = thread::spawn(reader(err, id.to_string(), "stderr"));
    process_io::write_stdin(&mut child, stdin);
    let (status, timed_out) = process_wait::wait(child, timeout)?;
    let stdout = process_io::join(out_reader, "stdout")?;
    let stderr = process_io::join(err_reader, "stderr")?;
    Ok(process_result::value(status, timed_out, stdout, stderr))
}

fn child(command: String, args: Vec<String>, has_stdin: bool) -> Result<Child, String> {
    Command::new(&command)
        .args(args)
        .stdin(if has_stdin { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("process_run: spawn {command} failed: {e}"))
}

fn reader<R: Read + Send + 'static>(r: R, id: String, stream: &'static str) -> impl FnOnce() -> std::io::Result<PipeOutput> {
    move || process_output::read(r, id, stream)
}