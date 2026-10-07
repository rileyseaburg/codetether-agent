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

pub(crate) mod cognition {
    pub use codetether_agent::cognition::{CandleDevicePreference, ThinkerBackend, ThinkerConfig};
}

#[path = "../src/cognition/thinker/bedrock_request.rs"]
mod bedrock_request;
#[path = "harness_identity/functiongemma.rs"]
mod functiongemma;
#[path = "harness_identity/gemini.rs"]
mod gemini;
#[path = "../src/provider/metrics/identity.rs"]
pub(crate) mod identity;
#[path = "../src/cognition/thinker/candle_dispatch/identity.rs"]
mod native_identity;
#[path = "../src/cognition/thinker/candle_runtime/identity.rs"]
mod native_model_identity;
#[path = "../src/cognition/thinker/openai_backend_request.rs"]
mod openai_backend_request;
#[path = "../src/cognition/thinker/openai_wire/mod.rs"]
mod openai_wire;
#[path = "harness_identity/routing.rs"]
mod routing;
