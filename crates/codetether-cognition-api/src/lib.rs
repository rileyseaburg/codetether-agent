//! Cognition HTTP projection, query ordering and event transport.
//!
//! HTTP routes use caller-supplied runtime interfaces. Response projections
//! also accept captured records directly without depending on a live runtime.
mod beliefs;
mod filter;
pub mod http;
mod proposals;
mod records;
mod stream;
pub use beliefs::list_beliefs;
pub use filter::BeliefFilter;
pub use proposals::list_proposals;
pub use records::{belief, latest_snapshot, persona};
pub use stream::stream_cognition;
