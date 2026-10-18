//! TUI command launch settings connected to the shared startup lifecycle.
//!
//! [`ApplicationHost`] forwards resolved command settings to [`crate::run`]
//! using the application's loading, frame and runtime operations.
use crate::Host;
use codetether_a2a_http::peer_startup::SpawnOptions;
use codetether_config::AccessMode;
use std::{marker::PhantomData, path::PathBuf};

/// Bind a TUI startup host to the command-line launch contract.
/// # Examples
/// ```
/// # fn example<H: codetether_tui_startup::Host>() {
/// fn accepts_cli<T: codetether_tui_cli::Host>() {}
/// accepts_cli::<codetether_tui_startup::cli::ApplicationHost<H>>();
/// # }
/// ```
pub struct ApplicationHost<H>(PhantomData<fn() -> H>);

impl<H: Host> codetether_tui_cli::Host for ApplicationHost<H> {
    async fn run(
        project: Option<PathBuf>,
        allow_network: bool,
        a2a_options: Option<SpawnOptions>,
        access_mode: Option<AccessMode>,
        yolo: bool,
        session_id: Option<String>,
    ) -> anyhow::Result<()> {
        crate::run::<H>(
            project,
            allow_network,
            a2a_options,
            access_mode,
            yolo,
            session_id,
        )
        .await
    }
}
