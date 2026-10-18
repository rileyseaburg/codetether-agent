//! Human-readable description of the selected cleanup scope.
use crate::CleanupArgs;

pub(super) fn scope(args: CleanupArgs) -> &'static str {
    if args.worktrees_only {
        "worktrees only"
    } else {
        "worktrees and branches"
    }
}
