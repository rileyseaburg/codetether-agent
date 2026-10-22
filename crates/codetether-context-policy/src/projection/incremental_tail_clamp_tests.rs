//! Active-window clamping must remove whole exchanges and keep the latest outcome.
use super::*;
use crate::projection::incremental_clamp::clamp_and_recompute;

fn exchange(id: &str) -> [Message; 2] {
    [
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: id.into(),
                name: "read".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: id.into(),
                content: "evidence".into(),
            }],
        },
    ]
}

#[test]
fn overflow_drops_a_complete_old_exchange_and_preserves_latest_pair() {
    let mut messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "Use CLI".into(),
        }],
    }];
    messages.extend(exchange("old"));
    messages.extend(exchange("new"));
    let mut levels = vec![ResidencyLevel::Full; messages.len()];
    let mut origins = (0..messages.len()).map(MessageOrigin::Clone).collect();
    let (dropped, tags) = clamp_and_recompute(
        &mut messages,
        &mut levels,
        &mut origins,
        30,
        5,
        &[],
        |messages| messages.len() * 10,
    );
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[0].role, Role::User);
    let ContentPart::ToolCall { id, .. } = &messages[1].content[0] else {
        panic!("call retained")
    };
    let ContentPart::ToolResult { tool_call_id, .. } = &messages[2].content[0] else {
        panic!("result retained")
    };
    assert_eq!(id, "new");
    assert_eq!(id, tool_call_id);
    assert_eq!(levels.len(), messages.len());
    assert_eq!(origins.len(), messages.len());
    assert_eq!(dropped, vec![(1, 3)]);
    assert!(tags.contains(&"incremental_active_history_clamp"));
    assert!(!drop_oldest_exchange(
        &mut messages,
        &mut levels,
        &mut origins
    ));
}

#[test]
fn noncontiguous_tool_results_are_not_split_by_tail_clamping() {
    let [call, result] = exchange("interleaved");
    let mut messages = vec![
        Message {
            role: Role::User,
            content: vec![],
        },
        call,
        Message {
            role: Role::User,
            content: vec![],
        },
        result,
    ];
    let mut levels = vec![ResidencyLevel::Full; messages.len()];
    let mut origins = (0..messages.len()).map(MessageOrigin::Clone).collect();
    assert!(!drop_oldest_exchange(
        &mut messages,
        &mut levels,
        &mut origins
    ));
    assert_eq!(messages.len(), 4);
}
