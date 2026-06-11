use std::path::Path;

use super::SandboxPolicy;

pub(super) fn covered(policy: &SandboxPolicy, path: &Path) -> bool {
    policy
        .allowed_paths
        .iter()
        .any(|allowed| path.starts_with(allowed))
}
