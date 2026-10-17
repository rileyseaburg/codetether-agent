//! Tests for workspace and attention contract defaults.

use chrono::Utc;

use super::{AttentionItem, AttentionSource, GlobalWorkspace};

#[test]
fn global_workspace_default() {
    let ws = GlobalWorkspace::default();
    assert!(ws.top_beliefs.is_empty());
    assert!(ws.top_uncertainties.is_empty());
    assert!(ws.top_attention.is_empty());
}

#[test]
fn attention_item_creation() {
    let item = AttentionItem {
        id: "a1".to_string(),
        topic: "test topic".to_string(),
        topic_tags: vec!["reliability".to_string()],
        priority: 0.8,
        source_type: AttentionSource::ContestedBelief,
        source_id: "b1".to_string(),
        assigned_persona: None,
        created_at: Utc::now(),
        resolved_at: None,
    };
    assert!(item.resolved_at.is_none());
    assert_eq!(item.priority, 0.8);
}
