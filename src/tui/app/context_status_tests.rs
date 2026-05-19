//! Tests for `/context status` rendering.

#[cfg(test)]
mod tests {
    use crate::tui::app::context_status::render;
    use crate::tui::app::state::AppState;

    #[test]
    fn render_includes_usage_and_recent_events() {
        let mut state = AppState {
            context_used: Some(50),
            context_budget: Some(100),
            ..Default::default()
        };
        state.context_health.last_rlm = Some("done".to_string());
        let body = render(&state);
        assert!(body.contains("50.0%"));
        assert!(body.contains("Last RLM: done"));
    }
}
