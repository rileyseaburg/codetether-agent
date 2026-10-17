//! Response wire data and completion interpretation.
//!
//! `DsResponse` represents decoded server data; `parse` produces the shared
//! completion result while preserving usage and tool-call stop precedence.
mod envelope;
mod message;
mod usage;
mod parse;
mod finish;
pub(crate) use envelope::DsResponse;
use message::{DsMessage, DsToolCall, DsFunction};
use usage::DsUsage;
pub(crate) use parse::parse;
