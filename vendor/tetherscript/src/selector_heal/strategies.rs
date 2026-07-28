//! Healing strategies.

mod attr;
mod proximity;
mod role;
mod structural;
mod text;

pub use attr::attr_recovery;
pub use proximity::{position_hint, sibling_proximity};
pub use role::role_based;
pub use structural::structural_path;
pub use text::text_proximity;
