//! Pure message conversion for a reasoning-repair hook.
//!
//! [build_ds_msg] forms the hook JSON payload; [apply_ds_repair] replaces
//! thinking blocks from the repaired reasoning text. Plugin execution belongs
//! to the host and is not performed by this module.
use codetether_provider_core::{ContentPart, Message, Role};
use serde_json::Value;
mod build_ds_msg;
pub use build_ds_msg::build_ds_msg;
mod role_str;
use role_str::role_str;
mod collect_text;
use collect_text::collect_text;
mod collect_reasoning;
use collect_reasoning::collect_reasoning;
mod collect_tool_calls;
use collect_tool_calls::collect_tool_calls;
mod apply_ds_repair;
pub use apply_ds_repair::apply_ds_repair;
