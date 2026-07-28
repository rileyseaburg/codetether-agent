//! Git status data for first-class TUI rendering.

#[path = "git_tui/model.rs"]
mod model;
#[path = "git_tui/parse.rs"]
mod parse;
#[path = "git_tui/render.rs"]
mod render;
#[path = "git_tui/repo.rs"]
mod repo;

#[allow(unused_imports)]
pub use model::GitPanel;
pub use render::render_panel;
pub use repo::load_panel;

#[cfg(test)]
#[path = "git_tui/tests.rs"]
mod tests;
