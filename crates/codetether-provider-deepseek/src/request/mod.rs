//! Request payloads, role conversion, and tool schemas.
//!
//! `build` converts a completion request to the provider payload. Child modules
//! preserve message order, reasoning/tool-call content, and tool definitions.
mod body;
mod convert;
mod convert_helpers;
mod convert_tools;
pub(crate) use body::build;
