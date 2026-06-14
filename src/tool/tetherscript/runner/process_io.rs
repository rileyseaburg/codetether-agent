use super::process_types::PipeOutput;
use std::io::Write;
use std::process::Child;
use std::thread;

pub fn write_stdin(child: &mut Child, stdin: Option<String>) {
    if let Some(input) = stdin {
        if let Some(mut pipe) = child.stdin.take() {
            let _ = pipe.write_all(input.as_bytes());
        }
    }
}

pub fn join(
    handle: thread::JoinHandle<std::io::Result<PipeOutput>>,
    stream: &str,
) -> Result<PipeOutput, String> {
    handle
        .join()
        .map_err(|_| format!("process_run: {stream} reader panicked"))?
        .map_err(|e| e.to_string())
}
