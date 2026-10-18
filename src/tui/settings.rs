//! Application binding for the shared settings presentation.
use crate::tui::{app::state::AppState, status::render_status};
use ratatui::{Frame, layout::Rect};
#[path = "settings_lines.rs"]
mod lines;
pub fn render_settings(f: &mut Frame, area: Rect, app_state: &AppState) {
    codetether_settings_view::render_settings(
        f,
        area,
        lines::view(app_state),
        &app_state.status,
        render_status,
    );
}

#[cfg(test)]
mod tests {
    use super::lines::settings_lines;
    use crate::tui::app::state::AppState;

    #[test]
    fn settings_panel_mentions_adjustable_controls() {
        let text = settings_lines(&AppState::default())
            .into_iter()
            .map(|line| {
                line.spans
                    .into_iter()
                    .map(|span| span.content.into_owned())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(text.contains("Edit auto-apply"));
        assert!(text.contains("Access mode"));
        assert!(text.contains("Bedrock thinking effort"));
        assert!(text.contains("OpenAI Codex thinking effort"));
        assert!(text.contains("OpenRouter thinking effort"));
        assert!(text.contains("Up / Down selects a setting"));
    }
}
