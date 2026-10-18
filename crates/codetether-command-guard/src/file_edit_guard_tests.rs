//! File-edit guard regression coverage.
use super::file_edit_guard_reason;
#[path = "file_edit_guard_tests/allows_read_only_usage.rs"]
mod allows_read_only_usage;
#[path = "file_edit_guard_tests/blocks_cat_heredoc.rs"]
mod blocks_cat_heredoc;
#[path = "file_edit_guard_tests/blocks_cat_redirect.rs"]
mod blocks_cat_redirect;
#[path = "file_edit_guard_tests/blocks_inline_interpreter_programs_even_without_write_calls.rs"]
mod blocks_inline_interpreter_programs_even_without_write_calls;
#[path = "file_edit_guard_tests/blocks_python_write.rs"]
mod blocks_python_write;
#[path = "file_edit_guard_tests/blocks_sed_in_place.rs"]
mod blocks_sed_in_place;
#[path = "file_edit_guard_tests/blocks_tee_write.rs"]
mod blocks_tee_write;
