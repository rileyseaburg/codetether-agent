//! Compatibility exports for shared thinker retry classification.
pub(super) use codetether_cognition_core::thinker::http::retry::{
    is_transient_http_error, is_transient_reqwest_error,
};
#[cfg(test)]
#[path = "retry_tests.rs"]
mod tests;
