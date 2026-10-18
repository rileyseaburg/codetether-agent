//! Capture into a private directory, keeping only successful original images.

use super::{backend::Backend, command, output, session::Session};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

pub(super) async fn capture(
    session: Session,
    backends: &[(Backend, PathBuf)],
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    anyhow::ensure!(
        session != Session::Headless,
        "Linux desktop screenshot requires a graphical session (DISPLAY or WAYLAND_DISPLAY). Start CodeTether inside the logged-in desktop; a headless/SSH service cannot capture it automatically."
    );
    anyhow::ensure!(
        !backends.is_empty(),
        "No compatible Linux screenshot utility installed. GNOME: gnome-screenshot; KDE: spectacle; wlroots Wayland: grim; X11: scrot, maim, or ImageMagick import. Install the utility for your desktop and run computer_use status again."
    );
    let directory = super::artifact::directory()?;
    let path = directory.path().join("desktop.png");
    let mut failures = Vec::new();
    for (backend, program) in backends {
        match attempt(session, *backend, program, &path).await {
            Ok(result) => {
                let _ = directory.keep();
                return Ok(result);
            }
            Err(error) => failures.push(format!("{}: {error:#}", backend.name())),
        }
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }
    anyhow::bail!(
        "Linux desktop capture failed: {}. Wayland compositors may deny noninteractive screenshots; use the desktop's supported capture utility and grant any requested permission.",
        failures.join("; ")
    )
}

async fn attempt(
    session: Session,
    backend: Backend,
    program: &Path,
    path: &Path,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    command::run(session, backend, program, path, Duration::from_secs(10)).await?;
    let path = path.to_owned();
    tokio::task::spawn_blocking(move || output::result(&path, session, backend)).await?
}
