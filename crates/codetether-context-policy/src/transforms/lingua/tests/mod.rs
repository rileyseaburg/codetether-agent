//! lingua::tests transform rules.

use super::*;
mod asst;
use asst::asst;
mod user;
use user::user;
mod code_fences_are_preserved;
mod ellipsis_only_lines_removed;
mod recent_messages_are_untouched;
mod user_text_never_pruned;
