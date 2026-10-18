//! Stable registry keys for workspace-bound mux servers.

use std::path::Path;

use sha2::{Digest, Sha256};

/// Derive the record key for the server that owns `workspace`.
///
/// Keys are `ws-<12 hex>` so they satisfy [`super::validate_name`] and never
/// collide with user-chosen session names, which cannot start with `ws-` and
/// contain a hash they did not choose in practice; collisions are checked at
/// session creation regardless.
/// # Arguments
/// `workspace` is the path whose encoded OS bytes identify the server.
/// # Returns
/// A `ws-` prefix followed by twelve hexadecimal SHA-256 characters.
/// # Examples
/// ```
/// let key = codetether_mux_registry::for_workspace(std::path::Path::new("/repo"));
/// assert_eq!(key.len(), 15);
/// ```
pub fn for_workspace(workspace: &Path) -> String {
    let digest = Sha256::digest(workspace.as_os_str().as_encoded_bytes());
    format!("ws-{}", &hex::encode(digest)[..12])
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn keys_are_stable_and_workspace_specific() {
        let first = super::for_workspace(Path::new("/repo"));
        assert_eq!(first, super::for_workspace(Path::new("/repo")));
        assert_ne!(first, super::for_workspace(Path::new("/other")));
        assert!(super::super::validate_name(&first).is_ok());
    }
}
