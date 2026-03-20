use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::telemetry::PROVIDER_METRICS;
use crate::tui::app::state::App;
use crate::tui::chat::message::MessageType;

#[derive(Debug, Default)]
struct ToolLatencySummary {
    count: usize,
    failures: usize,
    total_ms: u64,
    max_ms: u64,
    last_ms: u64,
}

pub fn render_latency(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(area);

    let mut lines = vec![
        Line::from("Latency Inspector"),
        Line::from(""),
        Line::from("Process-local timing from the current TUI runtime."),
        Line::from("TTFT/TTL measure Enter to first/last assistant text observed in the TUI."),
        Line::from(format!("Workspace: {}", app.state.cwd_display)),
    ];

    if app.state.processing {
        let elapsed = app
            .state
            .current_request_elapsed_ms()
            .map(format_duration)
            .unwrap_or_else(|| "running".to_string());
        let ttft = app
            .state
            .current_request_first_token_ms
            .map(format_duration)
            .unwrap_or_else(|| "waiting".to_string());
        lines.push(Line::from(format!(
            "Current request: in flight for {elapsed} | TTFT {ttft}"
        )));
    } else {
        lines.push(Line::from("Current request: idle"));
    }

    match (
        app.state.last_request_first_token_ms,
        app.state.last_request_last_token_ms,
    ) {
        (Some(ttft_ms), Some(ttl_ms)) => lines.push(Line::from(format!(
            "Last request: TTFT {} | TTL {}",
            format_duration(ttft_ms),
            format_duration(ttl_ms)
        ))),
        (Some(ttft_ms), None) => lines.push(Line::from(format!(
            "Last request: TTFT {} | TTL unavailable",
            format_duration(ttft_ms)
        ))),
        (None, Some(ttl_ms)) => lines.push(Line::from(format!(
            "Last request: TTFT unavailable | TTL {}",
            format_duration(ttl_ms)
        ))),
        (None, None) => lines.push(Line::from("Last request: no token timing yet")),
    }

    if let Some(latency_ms) = app.state.last_completion_latency_ms {
        let model = app
            .state
            .last_completion_model
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let prompt = app.state.last_completion_prompt_tokens.unwrap_or_default();
        let output = app.state.last_completion_output_tokens.unwrap_or_default();
        lines.push(Line::from(format!(
            "Last model round-trip: {model} in {} ({} in / {} out)",
            format_duration(latency_ms),
            prompt,
            output
        )));
    } else {
        lines.push(Line::from("Last model round-trip: no request timing yet"));
    }

    if let Some(latency_ms) = app.state.last_tool_latency_ms {
        let tool = app
            .state
            .last_tool_name
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let outcome = if app.state.last_tool_success.unwrap_or(false) {
            "ok"
        } else {
            "fail"
        };
        lines.push(Line::from(format!(
            "Last tool: {tool} in {} [{outcome}]",
            format_duration(latency_ms)
        )));
    } else {
        lines.push(Line::from("Last tool: no timed tool executions yet"));
    }

    lines.push(Line::from(""));
    lines.push(section_heading("Provider Round-Trip Latency"));

    let mut provider_snapshots = PROVIDER_METRICS.all_snapshots();
    provider_snapshots.sort_by(|a, b| {
        b.request_count
            .cmp(&a.request_count)
            .then_with(|| b.avg_latency_ms.total_cmp(&a.avg_latency_ms))
            .then_with(|| a.provider.cmp(&b.provider))
    });
    if provider_snapshots.is_empty() {
        lines.push(Line::from("  No provider requests recorded yet."));
    } else {
        for snapshot in provider_snapshots.iter().take(6) {
            lines.push(Line::from(format!(
                "  {}: avg {} | p50 {} | p95 {} | {} reqs | {} avg TPS",
                snapshot.provider,
                format_duration(snapshot.avg_latency_ms.round() as u64),
                format_duration(snapshot.p50_latency_ms.round() as u64),
                format_duration(snapshot.p95_latency_ms.round() as u64),
                snapshot.request_count,
                format_tps(snapshot.avg_tps)
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(section_heading("Tool Latency In Chat"));

    let tool_summaries = summarize_tool_latencies(app);
    if tool_summaries.is_empty() {
        lines.push(Line::from(
            "  No timed tool executions recorded in the current chat buffer.",
        ));
    } else {
        for (tool_name, stats) in tool_summaries.iter().take(6) {
            let avg_ms = stats.total_ms / stats.count.max(1) as u64;
            lines.push(Line::from(format!(
                "  {}: avg {} | max {} | {} runs | {} failures",
                tool_name,
                format_duration(avg_ms),
                format_duration(stats.max_ms),
                stats.count,
                stats.failures
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(section_heading("Recent Timed Tools"));
    let recent = recent_tool_latencies(app, 8);
    if recent.is_empty() {
        lines.push(Line::from("  No recent timed tools yet."));
    } else {
        for entry in recent {
            lines.push(Line::from(format!("  {entry}")));
        }
    }

    let widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Latency"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            " LATENCY ",
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Span::raw(" | "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Back | "),
        Span::styled("/chat", Style::default().fg(Color::Yellow)),
        Span::raw(": Return | "),
        Span::styled(
            app.state.status.clone(),
            Style::default().fg(if app.state.processing {
                Color::Yellow
            } else {
                Color::Green
            }),
        ),
    ]));
    f.render_widget(footer, chunks[1]);
}

fn section_heading(title: &str) -> Line<'static> {
    Line::from(vec![Span::styled(
        format!("  {title}"),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )])
}

fn summarize_tool_latencies(app: &App) -> Vec<(String, ToolLatencySummary)> {
    let mut by_tool: HashMap<String, ToolLatencySummary> = HashMap::new();

    for message in &app.state.messages {
        if let MessageType::ToolResult {
            name,
            success,
            duration_ms: Some(duration_ms),
            ..
        } = &message.message_type
        {
            let entry = by_tool.entry(name.clone()).or_default();
            entry.count += 1;
            entry.total_ms += *duration_ms;
            entry.max_ms = entry.max_ms.max(*duration_ms);
            entry.last_ms = *duration_ms;
            if !*success {
                entry.failures += 1;
            }
        }
    }

    let mut summaries: Vec<_> = by_tool.into_iter().collect();
    summaries.sort_by(|a, b| {
        let a_avg = a.1.total_ms / a.1.count.max(1) as u64;
        let b_avg = b.1.total_ms / b.1.count.max(1) as u64;
        b_avg
            .cmp(&a_avg)
            .then_with(|| b.1.last_ms.cmp(&a.1.last_ms))
            .then_with(|| a.0.cmp(&b.0))
    });
    summaries
}

fn recent_tool_latencies(app: &App, limit: usize) -> Vec<String> {
    app.state
        .messages
        .iter()
        .rev()
        .filter_map(|message| {
            if let MessageType::ToolResult {
                name,
                success,
                duration_ms: Some(duration_ms),
                ..
            } = &message.message_type
            {
                let icon = if *success { "OK" } else { "ER" };
                Some(format!("[{icon}] {name} {}", format_duration(*duration_ms)))
            } else {
                None
            }
        })
        .take(limit)
        .collect()
}

fn format_duration(ms: u64) -> String {
    match ms {
        0..=999 => format!("{ms}ms"),
        1_000..=9_999 => format!("{:.2}s", ms as f64 / 1_000.0),
        10_000..=59_999 => format!("{:.1}s", ms as f64 / 1_000.0),
        _ => format!("{:.1}m", ms as f64 / 60_000.0),
    }
}

fn format_tps(tps: f64) -> String {
    if tps >= 100.0 {
        format!("{tps:.0}")
    } else if tps >= 10.0 {
        format!("{tps:.1}")
    } else {
        format!("{tps:.2}")
    }
}
