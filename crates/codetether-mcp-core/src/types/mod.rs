//! MCP wire contracts shared by clients and servers.
//!
//! JSON-RPC envelopes, initialization, tool, resource, prompt and sampling
//! records preserve their protocol field names and constructors.
use serde::{Deserialize, Serialize};
use serde_json::Value;
include!("declarations.rs");
include!("exports.rs");
