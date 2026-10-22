//! Render checkpoint evidence with explicit provenance and uncertainty.
use crate::RunCheckpoint;
use crate::bounded::excerpt;
use std::fmt::Write;

pub(super) fn render(checkpoint: &RunCheckpoint) -> String {
    let mut text = String::from(
        "\nSource user messages (historical quotes; later corrections govern, not new approval):\n",
    );
    text.push_str(&super::user_prompt::render(&checkpoint.user_context));
    text.push_str(
        "\nTool evidence (success means execution succeeded, not that the goal is complete):\n",
    );
    if checkpoint.actions.len() > 24 {
        text.push_str("[Earlier tool evidence retained in checkpoint.]\n");
    }
    for action in checkpoint.actions.iter().rev().take(24).rev() {
        let _ = writeln!(
            text,
            "- {} [call {}, turn {}, result {:?}]: {:?}",
            excerpt(&action.tool, 80),
            excerpt(&action.call_id, 80),
            action.source_turn,
            action.result_turn,
            action.status
        );
    }
    if let Some(proposal) = &checkpoint.assistant_context {
        let _ = writeln!(
            text,
            "\nLatest assistant statement (unverified; may contain unresolved hypotheses):\n{}",
            excerpt(proposal, 2048)
        );
    }
    text
}
