use ratatui::text::Line;

use super::super::policy::BYTE_LIMIT;
use super::FormatCache;

#[test]
fn oversized_render_is_not_retained() {
    let mut cache = FormatCache::new();
    cache.insert((1, 2, 3, 80, 2), vec![Line::from("x".repeat(BYTE_LIMIT))]);
    assert!(cache.get(&(1, 2, 3, 80, 2)).is_none());
    assert_eq!(cache.bytes, 0);
}

#[test]
fn cumulative_render_weight_evicts_old_entries() {
    let mut cache = FormatCache::new();
    let content = "x".repeat(BYTE_LIMIT / 2);
    cache.insert((1, 2, 3, 80, 2), vec![Line::from(content.clone())]);
    cache.insert((2, 3, 4, 80, 2), vec![Line::from(content)]);
    assert!(cache.get(&(1, 2, 3, 80, 2)).is_none());
    assert!(cache.get(&(2, 3, 4, 80, 2)).is_some());
    assert!(cache.bytes <= BYTE_LIMIT);
}
