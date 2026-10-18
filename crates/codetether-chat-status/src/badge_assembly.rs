//! Ordered session, trust, bus, attachment, timing and agent badges.
use crate::status_hints::session_label_spans;
use ratatui::text::Span;

/// Assemble status badges in their display order.
///
/// # Arguments
/// * `source` - Provider of live badge components.
/// * `session_label` - Current session label.
///
/// # Returns
/// Owned badge spans with separators.
///
/// # Examples
/// ```
/// # fn example<T: codetether_chat_status::BadgeSource>(source: &T) {
/// let _badges = codetether_chat_status::badge_spans(source, "session");
/// # }
/// ```
pub fn badge_spans(source: &impl crate::BadgeSource, session_label: &str) -> Vec<Span<'static>> {
    let mut spans = session_label_spans(session_label);
    spans.extend(source.auto_apply_spans());
    push_optional_badge(&mut spans, source.yolo_badge());
    push_optional_badge(&mut spans, source.trust_status_badge_span());
    spans.push(Span::raw(" | "));
    spans.push(source.bus_status_badge_span());
    push_optional_badge(&mut spans, source.pending_images_badge());
    if let Some(latency) = source.latency_badge_spans() {
        spans.push(Span::raw(" | "));
        spans.extend(latency);
    }
    push_optional_badge(&mut spans, source.turn_latency_badge());
    let rail = source.agent_rail_spans();
    if !rail.is_empty() {
        spans.push(Span::raw(" | "));
        spans.extend(rail);
    }
    spans
}

fn push_optional_badge(spans: &mut Vec<Span<'static>>, badge: Option<Span<'static>>) {
    if let Some(badge) = badge {
        spans.push(Span::raw(" | "));
        spans.push(badge);
    }
}
