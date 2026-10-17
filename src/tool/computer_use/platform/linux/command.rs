//! Bounded, shell-free capture execution; cancellation terminates its process tree.

use super::{backend::Backend, session::Session};
use anyhow::Context;
use std::{path::Path, process::Stdio, time::Duration};

pub(super) async fn run(
    session: Session,
    backend: Backend,
    program: &Path,
    path: &Path,
    limit: Duration,
) -> anyhow::Result<()> {
    let mut command = tokio::process::Command::new(program);
    command
        .args(backend.arguments())
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if session == Session::Wayland {
        command
            .env_remove("DISPLAY")
            .env("GDK_BACKEND", "wayland")
            .env("QT_QPA_PLATFORM", "wayland");
    }
    crate::tool::process_tree::configure(&mut command);
    let mut child = command
        .spawn()
        .with_context(|| format!("Start {}", backend.name()))?;
    let mut guard = crate::tool::process_tree::Guard::attach(&child);
    let status = tokio::time::timeout(limit, child.wait())
        .await
        .with_context(|| {
            format!(
                "{} timed out after {}s; check desktop capture permissions",
                backend.name(),
                limit.as_secs()
            )
        })??;
    anyhow::ensure!(
        status.success(),
        "{} exited with {status}; check desktop capture permissions",
        backend.name()
    );
    guard.disarm();
    Ok(())
}
