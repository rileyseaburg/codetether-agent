//! Return to the original branch through the application policy guard.

use crate::Host;
use std::path::Path;
pub(crate) fn branch<H: Host>(
    cwd: &Path,
    starting_branch: &Option<String>,
    feature_branch: &str,
) -> bool {
    if let Some(start) = starting_branch {
        if !feature_branch.is_empty() && start != feature_branch {
            let _ = H::checkout(cwd, start);
            true
        } else {
            false
        }
    } else {
        false
    }
}
