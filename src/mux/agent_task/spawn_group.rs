//! Process-group isolation so cancellation reaches task-owned tool children.

#[cfg(unix)]
pub(super) fn isolate(command: &mut tokio::process::Command) {
    use std::os::unix::process::CommandExt;

    command.as_std_mut().process_group(0);
}

#[cfg(not(unix))]
pub(super) fn isolate(_: &mut tokio::process::Command) {}
