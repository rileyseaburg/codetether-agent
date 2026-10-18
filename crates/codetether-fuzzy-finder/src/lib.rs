//! Workspace fuzzy file matching and modal presentation.
//!
//! [scan_workspace] discovers [FuzzyEntry] candidates. [FuzzyFindState] owns
//! query and ranked results; [render] and [render_if_active] draw the picker.
//! Editor opening and cancellation status remain host responsibilities.
mod area;
mod entry;
mod lifecycle;
mod optional;
mod rank;
mod render;
mod row;
mod scan;
mod state;
mod window;
pub use entry::FuzzyEntry;
pub use optional::render_if_active;
pub use rank::rerank_into;
pub use render::render;
pub use scan::scan_workspace;
pub use state::FuzzyFindState;

mod input;
mod input_action;
mod selection;
mod take_selection;
pub use input_action::FinderAction;
pub use take_selection::take_selected_path;
