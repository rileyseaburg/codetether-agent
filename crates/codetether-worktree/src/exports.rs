//! Public worktree manager, result and TUI-state compatibility exports.
pub use crate::delivery::{Delivery, DeliveryStatus, deliver_branch, ledger_path};
pub use crate::info::{MergeResult, WorktreeInfo};
pub use crate::manager::WorktreeManager;
pub use crate::tui_active::{is_tui_active, set_tui_active};
