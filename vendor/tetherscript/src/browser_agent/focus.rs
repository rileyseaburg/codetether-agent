//! Focus traversal support for agent-controlled pages.

#[path = "focus_model.rs"]
mod model;
#[path = "focus_move.rs"]
mod movement;
#[path = "focus_order.rs"]
mod order;
#[path = "focus_path.rs"]
mod path;
#[path = "focus_script.rs"]
mod script;

pub use model::FocusTarget;
pub(crate) use path::selector_for;
