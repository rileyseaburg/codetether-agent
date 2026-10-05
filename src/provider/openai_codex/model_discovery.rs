//! Live model discovery for the ChatGPT-backed Codex provider.
//!
//! The Codex backend publishes the models an account may use at
//! `GET {CHATGPT_CODEX_API_URL}/models?client_version=...` (the same
//! endpoint the official Codex CLI queries). Results are cached
//! process-wide so synchronous callers (router candidates, autochat
//! rotation, smart switch) see the discovered catalog. The static
//! [`super::model_catalog`] list is only an offline seed used before the
//! first successful discovery or when the account is not authenticated.

#[path = "model_discovery_cache.rs"]
mod cache;
#[path = "model_discovery_wire.rs"]
mod wire;

pub(crate) use cache::{discovered, discovered_slugs, model, store};
pub(crate) use wire::{DiscoveredModel, parse_models};

/// How long a successful discovery stays fresh before re-fetching.
pub(crate) const TTL: std::time::Duration = std::time::Duration::from_secs(600);
