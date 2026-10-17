//! Execute the existing computer backend on the current or a temporary runtime.
use crate::Host;

pub(super) fn run_dispatch<H: Host>(
    input: &H::Input,
) -> Result<codetether_tool_core::ToolResult, String> {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(H::dispatch(input)),
        Err(_) => runtime_dispatch::<H>(input),
    }
    .map_err(|e| format!("computer: computer_use dispatch failed: {e}"))
}

fn runtime_dispatch<H: Host>(input: &H::Input) -> anyhow::Result<codetether_tool_core::ToolResult> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(H::dispatch(input))
}
