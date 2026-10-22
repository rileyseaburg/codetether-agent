//! Session and prepared-index access for shared incremental context derivation.
use crate::session::Session;
#[cfg(test)]
#[path = "incremental_required.rs"]
mod required;

codetether_incremental_derivation::impl_session_source!(
    Session,
    crate::session::index_produce::proactive::prepared_index,
    Session::message_offset
);
