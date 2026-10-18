//! Credential-free regressions independent of the server's library-test modules.
//! Production sources are included so their private request helpers remain testable.

pub(crate) mod provider {
    pub use codetether_agent::provider::*;

    pub(crate) mod metrics {
        pub(crate) use crate::identity;
        pub use codetether_agent::provider::metrics::*;
    }
}

use codetether_agent::cognition::ThinkerConfig;
pub(crate) use codetether_cognition_core::thinker;

pub(crate) mod cognition {
    pub use codetether_agent::cognition::{CandleDevicePreference, ThinkerBackend, ThinkerConfig};
}

#[path = "../../codetether-cognition-core/src/thinker/protocol/bedrock_request.rs"]
mod bedrock_request;
#[path = "harness_identity/functiongemma.rs"]
mod functiongemma;
#[path = "harness_identity/gemini.rs"]
mod gemini;
pub(crate) use codetether_provider_core::request_identity as identity;
#[path = "../../codetether-cognition-native/src/candle_dispatch/identity.rs"]
mod native_identity;
#[path = "../../codetether-cognition-native/src/candle_runtime/identity.rs"]
mod native_model_identity;
#[path = "../../codetether-cognition-core/src/thinker/protocol/openai_request.rs"]
mod openai_backend_request;
use codetether_cognition_core::thinker::protocol::openai_wire;
#[path = "harness_identity/routing.rs"]
mod routing;
