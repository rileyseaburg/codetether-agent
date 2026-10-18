//! Capture and present workspace Git state in terminal views.
//!
//! [capture_git_state] collects status, logs, diffs and branches;
//! [GitViewState] stores the snapshot. [render_git_view] presents that snapshot
//! with a caller-provided input-preview renderer.
mod branch_label;
mod branches;
mod capture;
mod diff;
mod lines;
mod log;
mod render;
mod sections;
mod state;
mod status;
mod style;
pub use branches::capture_branches;
pub use capture::capture_git_state;
pub use diff::{capture_diff_stat, capture_file_diff};
pub use lines::build_git_lines;
pub use log::capture_log;
pub use render::render_git_view;
pub use state::GitViewState;
pub use status::detect_git_status;
pub use style::heading;
