//! Compatibility exports for shared request-routing identity.
#[cfg(test)]
use codetether_provider_core::request_identity::{END, START};
pub(crate) use codetether_provider_core::request_identity::{
    caller_prompt, inject, prompt, system_prompt,
};
#[cfg(test)]
#[path = "identity_tests/system_tests.rs"]
mod system_tests;
#[cfg(test)]
#[path = "identity_tests/tests.rs"]
mod tests;
