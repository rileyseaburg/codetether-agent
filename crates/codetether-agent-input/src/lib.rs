//! Agent-tool payloads and borrowed spawn request validation.
//!
//! Deserialize [Params] using the existing wire-field names, then use
//! [SpawnRequest::from_params] for spawn-specific required fields and model
//! fallback. Helpers preserve synchronous detach defaults and construct the
//! shared resume configuration without executing an agent or loading a session.
mod params;
mod params_methods;
mod spawn_parse;
mod spawn_request;
pub use params::Params;
pub use spawn_request::SpawnRequest;
#[cfg(test)]
mod params_tests;
#[cfg(test)]
mod spawn_context_tests;
#[cfg(test)]
mod spawn_detach_tests;
#[cfg(test)]
mod spawn_model_tests;
#[cfg(test)]
mod test_helpers;
