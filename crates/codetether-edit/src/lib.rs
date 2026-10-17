//! Confirmation previews and proposed contents for regular edit operations.
//!
//! [EditTool] reads a target, prepares its replacement and returns confirmation
//! metadata. [proposed] reconstructs contents for approval-time diagnostics.
//! Exact and whitespace matching share one private matching plan.
//!
//! ```rust
//! # fn example<E: codetether_morph_backend::Editor + Default>() {
//! let _tool = codetether_edit::EditTool::<E>::new();
//! # }
//! ```
mod args;
mod diff;
mod execute;
mod matcher;
mod metadata;
mod morph;
mod morph_flow;
mod morph_infer;
pub mod proposed;
mod schema;
mod tool_struct;
pub use tool_struct::EditTool;
