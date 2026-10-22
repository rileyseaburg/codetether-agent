use super::*;

#[test]
fn reply_route_stays_open_until_all_inbound_turns_finish() {
    let peer = format!("requester-{}", uuid::Uuid::new_v4());
    let first = begin_reply(&peer);
    let second = begin_reply(&peer);
    assert!(is_waiting(&peer));
    drop(first);
    assert!(is_waiting(&peer));
    drop(second);
    assert!(!is_waiting(&peer));
}

#[tokio::test]
async fn headless_errors_release_the_reply_route() {
    let peer = format!("failed-requester-{}", uuid::Uuid::new_v4());
    let tasks = DashMap::new();
    let result =
        crate::inbound_turn::execute_turn(&tasks, "task", None, &peer, "question", async {
            assert!(is_waiting(&peer));
            anyhow::bail!("failed turn")
        })
        .await;
    assert!(result.is_err());
    assert!(!is_waiting(&peer));
}

#[test]
fn inbound_prompt_preserves_request_and_explains_direct_replies() {
    let message = prompt("requester", "Please review this change");
    assert!(message.contains("returned directly"));
    assert!(message.contains("clarification questions"));
    assert!(message.contains("Do not open another agent message to @requester"));
    assert!(message.ends_with("Please review this change"));
}
