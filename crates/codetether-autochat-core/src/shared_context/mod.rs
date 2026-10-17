//! Shared relay memory and deterministic context conversion policy.
//!
//! [ContextDelta] messages update [SharedRelayContext]; [ContextDeltaEnvelope]
//! associates updates with a relay and turn. Publish and drain updates through
//! the shared bus, then render the current state into the next handoff prompt.
use codetether_bus_core::{BusHandle, BusMessage};
use codetether_rlm::FinalPayload;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

const CONTEXT_KEY_PREFIX: &str = "relay.";

const CONTEXT_BUCKET_LIMIT: usize = 12;
mod context_delta;
pub use context_delta::ContextDelta;
mod shared_relay_context;
pub use shared_relay_context::SharedRelayContext;
mod context_delta_envelope;
pub use context_delta_envelope::ContextDeltaEnvelope;
include!("modules.rs");
