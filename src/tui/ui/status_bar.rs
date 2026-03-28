use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::tui::app::state::App;

pub fn format_timestamp(timestamp: SystemTime) -> String {
    let secs = timestamp
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let day = secs % 86_400;
    let hour = day / 3_600;
    let minute = (day % 3_600) / 60;
    let second = day % 60;
    format!("{hour:02}:{minute:02}:{second:02}")
}

pub fn bus_status_label_and_color(app: &App) -> (String, Color) {
    let connected = app.state.a2a_connected;
    let agents = app.state.worker_bridge_registered_agents.len();
    let queued = app.state.worker_task_queue.len();
    let processing = app
        .state
        .worker_bridge_processing_state
        .unwrap_or(app.state.processing);

    if connected || agents > 0 || queued > 0 {
        let state = if processing { "active" } else { "idle" };
        (
            format!("BUS {state} ({agents}ag/{queued}q)"),
            if processing {
                Color::Green
            } else {
                Color::Yellow
            },
        )
    } else {
        ("BUS offline".to_string(), Color::DarkGray)
    }
}

pub fn bus_status_badge_span(app: &App) -> Span<'static> {
    let (label, color) = bus_status_label_and_color(app);
    Span::styled(
        format!(" {label} "),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}

pub fn latency_badge_spans(app: &App) -> Option<Vec<Span<'static>>> {
    let mut spans = Vec::new();

    if app.state.processing {
        spans.push(request_timing_span(
            "TTFT",
            app.state.current_request_first_token_ms,
            Color::Cyan,
            true,
        ));
        if let Some(elapsed_ms) = app.state.current_request_elapsed_ms() {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                format!(" ELAPSED {} ", format_duration_ms(elapsed_ms)),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
        }
        return Some(spans);
    }

    if app.state.last_request_first_token_ms.is_none()
        && app.state.last_request_last_token_ms.is_none()
    {
        return None;
    }

    spans.push(request_timing_span(
        "TTFT",
        app.state.last_request_first_token_ms,
        Color::Cyan,
        false,
    ));
    spans.push(Span::raw(" "));
    spans.push(request_timing_span(
        "TTL",
        app.state.last_request_last_token_ms,
        Color::Green,
        false,
    ));
    Some(spans)
}

fn request_timing_span(
    label: &str,
    duration_ms: Option<u64>,
    color: Color,
    emphasize: bool,
) -> Span<'static> {
    let display = duration_ms
        .map(format_duration_ms)
        .unwrap_or_else(|| "…".to_string());
    let mut style = Style::default().fg(color);
    if emphasize {
        style = style.add_modifier(Modifier::BOLD);
    }
    Span::styled(format!(" {label} {display} "), style)
}

pub fn format_duration_ms(duration_ms: u64) -> String {
    if duration_ms >= 60_000 {
        format!(
            "{}m{:02}s",
            duration_ms / 60_000,
            (duration_ms % 60_000) / 1_000
        )
    } else if duration_ms >= 1_000 {
        format!("{:.1}s", duration_ms as f64 / 1_000.0)
    } else {
        format!("{duration_ms}ms")
    }
}

pub fn session_model_label(state: &crate::tui::app::state::AppState) -> Option<String> {
    if state.processing {
        Some("processing".to_string())
    } else {
        None
    }
}
