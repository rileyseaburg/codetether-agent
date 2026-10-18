//! Format status report sections from a borrowed snapshot.
use super::Snapshot;

pub(super) fn tui(state: &Snapshot<'_>) -> String {
    format!(
        "TUI: auto_apply={} network={} worktree={} autocomplete={}",
        state.auto_apply_edits, state.allow_network, state.use_worktree, state.slash_autocomplete,
    )
}

pub(super) fn context(state: &Snapshot<'_>) -> String {
    match (state.context_used, state.context_budget) {
        (Some(used), Some(budget)) => format!("Context: {used}/{budget} tokens"),
        _ => "Context: unavailable".to_string(),
    }
}

pub(super) fn completion(state: &Snapshot<'_>) -> String {
    let Some(model) = state.last_completion_model else {
        return "Last completion: none".to_string();
    };
    let prompt = state.last_completion_prompt_tokens.unwrap_or(0);
    let output = state.last_completion_output_tokens.unwrap_or(0);
    format!("Last completion: {model} prompt={prompt} output={output}")
}
