//! Mocked-local protocol/lifecycle tests, independent of native Windows APIs.
#[path = "tests/framing.rs"]
mod framing;
#[cfg(unix)]
#[path = "tests/late_reply.rs"]
mod late_reply;
#[cfg(unix)]
#[path = "tests/lifecycle.rs"]
mod lifecycle;
#[path = "tests/preflight.rs"]
mod preflight;
#[cfg(unix)]
#[path = "tests/recovery.rs"]
mod recovery;

#[cfg(unix)]
fn fixture(script: &str) -> Result<super::process::Process, super::failure::Failure> {
    let mut command = tokio::process::Command::new("/bin/sh");
    command.args(["-c", script]);
    super::process::Process::from_command(command)
}

#[cfg(unix)]
const ECHO: &str = r#"while IFS= read -r line; do printf '%s\n' '{"success":true,"output":"kept","metadata":{"remote":true}}'; done"#;
