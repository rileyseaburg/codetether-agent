//! Register read-only worker tools in their original order.
use crate::Host;
use std::path::{Path, PathBuf};
use std::sync::Arc;
/// Register read-only worker tools in their original order.
///
/// # Arguments
/// * `registry` — Registry receiving the ordered safe tools.
/// * `workspace_dir` — Workspace converted to the LSP file URI.
/// * `root_path` — Explicit root for glob, code search and todo reads.
/// * `H` — Application registry and constructor bindings.
/// # Returns
/// Registers this group in the supplied registry.
/// # Examples
/// See [crate] for a generic application-bound construction example.
pub fn register_safe_tools<H: Host>(
    registry: &mut H::Registry,
    workspace_dir: &Path,
    root_path: &PathBuf,
) {
    H::register(registry, Arc::new(codetether_file_tools::ReadTool::new()));
    H::register(registry, Arc::new(codetether_file_tools::ListTool::new()));
    H::register(
        registry,
        Arc::new(codetether_file_tools::GlobTool::with_root(
            root_path.clone(),
        )),
    );
    H::register(registry, Arc::new(codetether_grep_tool::GrepTool::new()));
    H::register(
        registry,
        H::lsp(format!("file://{}", workspace_dir.display())),
    );
    H::register(registry, Arc::new(codetether_webfetch::WebFetchTool::new()));
    H::register(
        registry,
        Arc::new(codetether_websearch::WebSearchTool::new()),
    );
    H::register(
        registry,
        Arc::new(codetether_code_search::CodeSearchTool::with_root(
            root_path.clone(),
        )),
    );
    H::register(
        registry,
        Arc::new(codetether_todo::TodoReadTool::with_root(root_path.clone())),
    );
    H::register(registry, Arc::new(codetether_skill_tool::SkillTool::new()));
}
