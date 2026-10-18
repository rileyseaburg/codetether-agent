//! Regression coverage for cache reuse, streaming invalidation and frozen rows.
use crate::{RenderInputs, RenderedLineCache};
use codetether_chat_model::StreamingState;
use ratatui::text::Line;

#[test]
fn idle_cache_is_reusable() {
    let mut cache = RenderedLineCache::default();
    let inputs = RenderInputs::default();
    cache.store_message_lines(vec![Line::from("cached")], 80, inputs);

    assert!(cache.is_message_cache_valid(80, inputs));
}

#[test]
fn streaming_replacement_invalidates_cached_suffix() {
    let mut cache = RenderedLineCache::default();
    let mut streaming = StreamingState::default();
    let inputs = RenderInputs {
        processing: true,
        ..Default::default()
    };
    cache.replace_streaming_text(&mut streaming, "first".to_string());
    cache.store_message_lines(vec![Line::from("first")], 80, inputs);
    assert!(cache.is_message_cache_valid(80, inputs));

    cache.replace_streaming_text(&mut streaming, "other".to_string());
    assert!(!cache.is_message_cache_valid(80, inputs));
}

#[test]
fn frozen_prefix_is_moved_without_retaining_streaming_suffix() {
    let mut cache = RenderedLineCache::default();
    let inputs = RenderInputs::default();
    cache.store_message_lines_with_frozen(
        vec![Line::from("history"), Line::from("preview")],
        80,
        1,
        inputs,
    );

    let frozen = cache.take_frozen_prefix(80, inputs).expect("frozen prefix");
    assert_eq!(frozen.len(), 1);
    assert!(cache.cached_message_lines.is_empty());
}
