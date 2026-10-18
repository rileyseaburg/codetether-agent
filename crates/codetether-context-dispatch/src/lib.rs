//! Select a context strategy and attach preserved session constraints.
//!
//! [derive_with_policy] executes legacy, reset or incremental derivation.
//! [effective_policy] applies shared policy precedence to native state.
//! [Source] supplies persisted constraints and offset information.
mod api;
mod execute;
mod incremental;
mod input;
mod legacy;
mod policy;
mod reset;
mod source;
mod state_header;
pub use api::derive_with_policy;
pub use policy::effective_policy;
pub use source::Source;
pub use state_header::prepend_state_header;
