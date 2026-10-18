//! Response wire data and completion interpretation.
//!
//! `DsResponse` represents decoded server data; `parse` produces the shared
//! completion result while preserving usage and tool-call stop precedence.
mod envelope;
mod finish;
mod message;
mod parse;
mod usage;
pub(crate) use envelope::DsResponse;
use message::DsMessage;
pub(crate) use parse::parse;
use usage::DsUsage;
