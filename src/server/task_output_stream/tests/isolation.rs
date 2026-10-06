use super::super::filter::matches_task;
use super::fixtures::{artifact, envelope, update};
use crate::bus::BusMessage;

#[test]
fn rejects_prefix_collisions_and_descendant_topics() {
    for id in ["abc-2", "abcd", "abc.output", "abc/other"] {
        let topic = format!("task.{id}");
        assert!(!matches_task(&update(&topic, id), "abc"));
        assert!(!matches_task(&artifact(&topic, id), "abc"));
        assert!(!matches_task(&update(&topic, "abc"), "abc"));
    }
    assert!(!matches_task(&update("task.10", "10"), "1"));
}

#[test]
fn requires_matching_payload_for_both_task_message_kinds() {
    assert!(matches_task(&update("task.abc", "abc"), "abc"));
    assert!(matches_task(&artifact("task.abc", "abc"), "abc"));
    for id in ["abc-2", "", "other"] {
        assert!(!matches_task(&update("task.abc", id), "abc"));
        assert!(!matches_task(&artifact("task.abc", id), "abc"));
    }
}

#[test]
fn rejects_non_task_messages_even_on_the_exact_topic() {
    let message = envelope(
        "task.abc",
        BusMessage::AgentShutdown {
            agent_id: "abc".into(),
        },
    );
    assert!(!matches_task(&message, "abc"));
    assert!(!matches_task(&update("agent.abc", "abc"), "abc"));
}
