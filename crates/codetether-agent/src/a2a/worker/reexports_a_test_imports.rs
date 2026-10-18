// Compatibility imports used by this module’s unit-test fixtures.
pub(super) use super::{
    a2a_peer::maybe_start_worker_a2a_peer,
    completion_retry::complete_worker_step_with_context_fallback,
    http_client::build_worker_http_client, metadata_flags::metadata_bool,
    metadata_numbers::metadata_usize,
};
