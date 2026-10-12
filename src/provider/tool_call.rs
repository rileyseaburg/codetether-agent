//! Compatibility export for the borrowed canonical tool-call accessor.
//!
//! [`ContentPart::as_tool_call`](super::ContentPart::as_tool_call) is implemented
//! in the core crate so its inherent implementation follows its owning type.

pub use codetether_provider_core::ToolCallRef;
