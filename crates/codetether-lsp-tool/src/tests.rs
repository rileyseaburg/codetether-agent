//! LSP argument and action alias regression tests.
#[path = "tests/accepts_path_alias_for_file_path.rs"]
mod accepts_path_alias_for_file_path;
#[path = "tests/maps_command_aliases_to_actions.rs"]
mod maps_command_aliases_to_actions;
#[path = "tests/parses_lsp_action_aliases.rs"]
mod parses_lsp_action_aliases;
#[path = "tests/rejects_unsupported_command_with_helpful_error.rs"]
mod rejects_unsupported_command_with_helpful_error;
