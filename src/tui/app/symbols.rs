mod gate;
mod languages;
pub mod mention;
mod parse;
mod rank;
mod refresh;
mod runtime;
mod search;

pub use refresh::{active as symbol_search_active, close as close_search};
pub use refresh::{drain as drain_refresh, schedule as schedule_refresh};

/// Cancel any pending query without closing the picker.
pub fn cancel_refresh() {
    runtime::cancel();
}
