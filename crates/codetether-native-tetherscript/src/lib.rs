//! Native TetherScript plugin execution with computer-use policy bindings.
//!
//! [TetherScriptPluginTool] dispatches plugins through [ApplicationHost], using
//! the shared computer backend and runtime permission checks. [convert] exposes
//! the runtime value conversions used by application consumers.
mod tetherscript;
pub use tetherscript::*;
#[cfg(test)]
mod tool {
    pub(crate) use crate::tetherscript;
    pub use codetether_tool_core::{Tool, ToolResult};
}
#[cfg(test)]
fn project_tool() -> TetherScriptPluginTool {
    TetherScriptPluginTool::with_root(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."),
    )
}
