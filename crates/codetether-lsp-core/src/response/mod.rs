//! Decoding of server replies into client-facing action results.
//!
//! The decoders accept [`JsonRpcResponse`] values and return [`LspActionResult`]
//! without accessing client state, processes, or transport. Protocol error
//! replies become error actions; individual decoders retain their existing
//! handling of absent, null, or malformed payloads.
//!
//! Use [`parse_completion_response`] for completion lists, [`parse_hover_response`]
//! for hover content, and the location and symbol decoders for navigation data.

use crate::types::{
    CompletionItemInfo, JsonRpcResponse, LocationInfo, LspActionResult, RangeInfo, SymbolInfo,
};
use anyhow::Result;

mod completion;
mod document_symbols;
mod hover;
mod location;
mod workspace_symbols;

pub use completion::parse_completion_response;
pub use document_symbols::parse_document_symbols_response;
pub use hover::parse_hover_response;
pub use location::parse_location_response;
pub use workspace_symbols::parse_workspace_symbols_response;
