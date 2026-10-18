//! Shared terminal view, input and theme data.
//!
//! [ViewMode] selects a major panel; [InputMode] describes keyboard handling;
//! [Theme] stores simple RGB accents. The public view-mode helper modules
//! expose the canonical registry, display names and shortcut help.
mod input_mode;
mod theme;
mod view_mode;
pub mod view_mode_display;
mod view_mode_display_name;
pub mod view_mode_help;
pub mod view_mode_registry;
mod view_mode_shortcut_hint;
/// Shared workspace entry and snapshot types for terminal views.
pub use codetether_workspace_snapshot::{WorkspaceEntry, WorkspaceEntryKind, WorkspaceSnapshot};
pub use input_mode::InputMode;
pub use theme::Theme;
pub use view_mode::ViewMode;

mod session_view;
pub use session_view::SessionView;
