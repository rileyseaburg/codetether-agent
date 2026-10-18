//! Register workspace-scoped editing tools.
use crate::Host;
use std::path::PathBuf;
use std::sync::Arc;
/// Register workspace-scoped editing tools.
///
/// # Arguments
/// * `registry` — Registry receiving editing tools.
/// * `root_path` — Root for shell execution, patches and todo writes.
/// * `H` — Application registry and constructor bindings.
/// # Returns
/// Registers this group in the supplied registry.
/// # Examples
/// See [crate] for a generic application-bound construction example.
pub fn register_edit_tools<H: Host>(registry: &mut H::Registry, root_path: PathBuf) {
    H::register(registry, Arc::new(codetether_file_tools::WriteTool::new()));
    H::register(
        registry,
        Arc::new(codetether_advanced_edit::AdvancedEditTool::new()),
    );
    H::register(
        registry,
        Arc::new(codetether_bash_tool::BashTool::with_cwd(root_path.clone())),
    );
    H::register(registry, H::multiedit());
    H::register(
        registry,
        Arc::new(codetether_patch::ApplyPatchTool::with_root(
            root_path.clone(),
        )),
    );
    H::register(
        registry,
        Arc::new(codetether_todo::TodoWriteTool::with_root(root_path)),
    );
    H::register(
        registry,
        Arc::new(codetether_confirm_edit::ConfirmEditTool::new()),
    );
    H::register(
        registry,
        Arc::new(codetether_confirm_multiedit::ConfirmMultiEditTool::new()),
    );
    H::register(registry, H::undo());
}
