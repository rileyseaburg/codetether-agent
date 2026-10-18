//! Boolean preference updates and session persistence through native adapters.
//!
//! Implement [`Session`] and [`View`], then call [`set`] for a preference update
//! or [`save`] after another settings mutation. Persistence failures become status text.
mod host;
mod preference;
mod save;
mod set;
pub use host::{Session, View};
pub use preference::Preference;
pub use save::save;
pub use set::set;
