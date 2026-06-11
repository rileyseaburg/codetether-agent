use crate::tui::app::state::App;

pub(super) fn tui(app: &App) -> String {
    format!(
        "TUI: auto_apply={} network={} worktree={} autocomplete={}",
        app.state.auto_apply_edits,
        app.state.allow_network,
        app.state.use_worktree,
        app.state.slash_autocomplete,
    )
}

pub(super) fn context(app: &App) -> String {
    match (app.state.context_used, app.state.context_budget) {
        (Some(used), Some(budget)) => format!("Context: {used}/{budget} tokens"),
        _ => "Context: unavailable".to_string(),
    }
}

pub(super) fn completion(app: &App) -> String {
    let Some(model) = app.state.last_completion_model.as_deref() else {
        return "Last completion: none".to_string();
    };
    let prompt = app.state.last_completion_prompt_tokens.unwrap_or(0);
    let output = app.state.last_completion_output_tokens.unwrap_or(0);
    format!("Last completion: {model} prompt={prompt} output={output}")
}
