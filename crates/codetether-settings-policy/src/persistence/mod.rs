//! Boolean preference updates and session persistence through native adapters.
//!
//! Implement [`Session`] and [`View`], then call [`set`] for a preference update
//! or [`save`] after another settings mutation. Persistence failures become status text.
//! [`Preferences`] owns interactive values; [`BorrowedView`] pairs them with status output.
mod borrowed_view;
mod host;
mod preference;
mod save;
mod set;
mod state;
pub use borrowed_view::BorrowedView;
pub use host::{Session, View};
pub use preference::Preference;
pub use save::save;
pub use set::set;
pub use state::Preferences;
