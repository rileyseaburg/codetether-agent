//! Shared A2A wire contracts independent of HTTP, gRPC and application state.
//!
//! Task, message, card, RPC, streaming, security and push modules retain their
//! serde representations. All public contracts are also exported at the root.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod tasks;
pub use tasks::*;
pub mod messages;
pub use messages::*;
pub mod cards;
pub use cards::*;
pub mod rpc;
pub use rpc::*;
pub mod security;
pub use security::*;
pub mod streaming;
pub use streaming::*;
pub mod push;
pub use push::*;

/// CodeTether identity extension on A2A agent cards.
pub mod identity_extension;

/// Default CodeTether card metadata and coding skills.
pub mod application_card;

/// JSON event payloads for A2A task observers.
pub mod observer_payloads;
