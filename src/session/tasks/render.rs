//! Render a [`TaskState`] as a system-prompt governance block.

use super::state::TaskState;

/// Format the goal + open tasks as a prompt block.
///
/// Returns `None` when there is no active goal **and** no open tasks
/// — in which case the governance middleware does not need to inject
/// anything. A default "credential safety" rule is always included
/// when a goal is present.
pub fn governance_block(state: &TaskState) -> Option<String> {
    if state.goal.is_none() && state.tasks.is_empty() {
        return None;
    }

    let mut out = String::from("## Goal Governance\n");

    if let Some(g) = &state.goal {
        out.push_str("\nOBJECTIVE: ");
        out.push_str(&g.objective);
        out.push('\n');

        if !g.success_criteria.is_empty() {
            out.push_str("\nDONE WHEN:\n");
            for c in &g.success_criteria {
                out.push_str("- ");
                out.push_str(c);
                out.push('\n');
            }
        }

        out.push_str("\nFORBIDDEN:\n");
        out.push_str(
            "- Entering credentials, passwords, or OAuth codes on behalf of the user. \
             Stop at login walls and hand control back.\n",
        );
        for f in &g.forbidden {
            out.push_str("- ");
            out.push_str(f);
            out.push('\n');
        }

        out.push_str(
            "\nIf the next action does not advance OBJECTIVE, stop and ask the user \
             before proceeding. Do not attempt to solve sub-problems that require \
             information you do not have.\n",
        );

        if state.tool_calls_since_reaffirm >= 10 || state.errors_since_reaffirm >= 3 {
            out.push_str(&format!(
                "\nDRIFT WARNING: {} tool calls and {} errors since last goal \
                 reaffirmation. State concretely what is done, what remains, or \
                 whether you are blocked — then call `session_task` with \
                 `action = \"reaffirm\"`.\n",
                state.tool_calls_since_reaffirm, state.errors_since_reaffirm,
            ));
        }
    }

    let open = state.open_tasks();
    if !open.is_empty() {
        out.push_str("\nOPEN TASKS:\n");
        for t in open {
            let marker = match t.status {
                super::event::TaskStatus::InProgress => "◐",
                _ => "○",
            };
            out.push_str(&format!("{} [{}] {}\n", marker, t.id, t.content));
        }
    }

    Some(out)
}
