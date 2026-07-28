//! JavaScript dialog records for agent-controlled pages.
//!
//! The dialog slice models `alert`, `confirm`, and `prompt` as deterministic
//! page records. Dialog decisions are queued before script execution so the
//! in-tree JavaScript runtime can return synchronously without blocking.

#[path = "dialog_codec.rs"]
mod dialog_codec;
#[path = "dialog_decisions.rs"]
mod dialog_decisions;
#[path = "dialog_model.rs"]
mod dialog_model;
#[path = "dialog_page.rs"]
mod dialog_page;
#[path = "dialog_parse.rs"]
mod dialog_parse;
#[path = "dialog_record_codec.rs"]
mod dialog_record_codec;
#[path = "dialog_records.rs"]
mod dialog_records;
#[path = "dialog_runtime.rs"]
mod dialog_runtime;
#[path = "dialog_script.rs"]
mod dialog_script;

pub use dialog_model::{DialogDecision, DialogKind, DialogRecord};
