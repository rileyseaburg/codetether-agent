//! Regression cases for dropped-history navigation.
use super::render_toc;
use codetether_provider_core::Role;
mod fixtures;
use fixtures::text;
mod toc_caps_entries_and_reports_overflow;
mod toc_empty_when_no_user_text;
mod toc_lists_user_turns_with_base_offset;
mod toc_truncates_long_first_lines;
