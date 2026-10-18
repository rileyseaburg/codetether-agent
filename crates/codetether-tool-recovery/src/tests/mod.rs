//! Original recovery-message and prose-extraction regressions.
mod extracts_backticked_command_from_prose;
mod extracts_command_after_run_verb_without_backticks;
mod returns_none_when_bash_tool_unavailable;
mod returns_none_when_prose_mentions_no_command;
mod support;
mod truncation;
