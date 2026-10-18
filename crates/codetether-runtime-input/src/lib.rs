//! Authoritative runtime identity and workspace fields for tool requests.
//!
//! [enrich_tool_input_with_runtime_context] normalizes workspace paths and adds
//! identity/provenance fields. [enrich] also applies a live prior-context ceiling;
//! [enrich_for_turn] adds the prompt-run lease owner. [Source] binds native sessions.
mod context;
mod fields;
mod path;
mod path_value;
mod provenance;
mod resolve;
mod session;
mod source;
pub use context::enrich_tool_input_with_runtime_context;
pub use fields::insert_field;
pub use path::normalize_workspace_paths;
pub use path_value::normalize_path_value;
pub use provenance::{insert_provenance_field, insert_provenance_fields};
pub use resolve::resolve_workspace_path;
pub use session::{enrich, enrich_for_turn, enrich_with_model};
pub use source::Source;
#[cfg(test)]
mod tests;

/// Implementation support for the application-local session binding.
#[doc(hidden)]
pub mod session_binding;
