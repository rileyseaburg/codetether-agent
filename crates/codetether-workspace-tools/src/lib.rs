//! Workspace-rooted tool installation and capability filtering.
//!
//! Implement [Registry] and [Host] for application bindings, then call [finish]
//! to replace workspace-sensitive tools and remove tools forbidden by capability.
mod aliases;
mod capability;
mod file_uri;
mod host;
mod policy;
mod readonly_bash;
mod root;
mod scoped_bash;
mod scoped_git;
mod scoped_tools;
pub use capability::Capability;
pub use host::{Host, Registry};
pub use root::resolve_root;
use std::path::Path;
/// Root tool instances in the canonical workspace and apply capability rules.
/// # Arguments
/// * `registry` — Existing catalog to scope and filter.
/// * `root` — Workspace directory, canonicalized when possible.
/// * `capability` — Assigned filesystem permissions and workflow restrictions.
/// # Returns
/// The same registry after rooted tool replacement and capability filtering.
/// # Examples
/// ```no_run
/// # fn scope<H: codetether_workspace_tools::Host>(registry: H::Registry) -> H::Registry {
/// codetether_workspace_tools::finish::<H>(registry, std::path::Path::new("."), codetether_workspace_tools::Capability::ReadOnly)
/// # }
/// ```
pub fn finish<H: Host>(
    mut registry: H::Registry,
    root: &Path,
    capability: Capability,
) -> H::Registry {
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    scoped_tools::install::<H>(&mut registry, &root, capability);
    policy::apply(&mut registry, capability);
    registry
}

/// Workspace catalogs and capability-appropriate coordination extensions.
pub mod composition;

/// Session workspace registration, prior-context filtering and autonomous mode.
pub mod session;
