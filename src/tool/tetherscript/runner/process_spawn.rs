use super::{
    ProcessGrant, process_io, process_output, process_result, process_types::PipeOutput,
    process_wait,
};
use std::io::Read;
use std::process::Child;
use std::thread;
use tetherscript::value::Value;

pub fn spawn(
    id: &str,
    grant: &ProcessGrant,
    command: String,
    args: Vec<String>,
    stdin: Option<String>,
    timeout: u64,
) -> Result<Value, String> {
    let mut child = child(grant, command, args, stdin.is_some())?;
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

fn child(
    grant: &ProcessGrant,
    command: String,
    args: Vec<String>,
    stdin: bool,
) -> Result<Child, String> {
    crate::tool::sandbox::sandbox_spawn_std::spawn(
        &command,
        &args,
        &grant.policy(),
        grant.workspace(),
        stdin,
    )
    .map_err(|e| format!("process_run: spawn {command} failed: {e}"))
}

fn reader<R: Read + Send + 'static>(
    r: R,
    id: String,
    stream: &'static str,
) -> impl FnOnce() -> std::io::Result<PipeOutput> {
    move || process_output::read(r, id, stream)
}
