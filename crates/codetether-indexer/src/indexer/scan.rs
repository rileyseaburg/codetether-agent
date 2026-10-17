//! scan responsibilities for codebase indexing.

use super::*;
mod should_descend;
pub(super) use should_descend::should_descend;
mod is_hidden_path;
pub(super) use is_hidden_path::is_hidden_path;
mod is_probably_text_file;
pub(super) use is_probably_text_file::is_probably_text_file;
