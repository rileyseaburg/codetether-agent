//! Pending edit confirmation execution and status formatting.
//!
//! Call [tool_result_requires_confirmation] to inspect preview metadata.
//! When application policy allows automatic application, use
//! [auto_apply_pending_confirmation]; otherwise format the pending status with
//! [pending_confirmation_tool_result_content]. Authorization stays with callers.
mod execute;
mod metadata;
mod render;
pub use execute::auto_apply_pending_confirmation;
pub use metadata::tool_result_requires_confirmation;
pub use render::{
    auto_apply_pending_confirmation_result_content, pending_confirmation_tool_result_content,
};

mod policy;
pub use policy::auto_apply_enabled;
