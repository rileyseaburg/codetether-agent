//! File-edit guard regression coverage.
use super::file_edit_guard_reason;
mod allows_read_only_usage;
mod blocks_cat_heredoc;
mod blocks_cat_redirect;
mod blocks_inline_interpreter_programs_even_without_write_calls;
mod blocks_python_write;
mod blocks_sed_in_place;
mod blocks_tee_write;
