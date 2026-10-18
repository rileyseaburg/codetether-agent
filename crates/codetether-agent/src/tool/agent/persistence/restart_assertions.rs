//! Durable FIFO assertions shared by restart recovery tests.

use crate::tool::agent::{collaboration_runtime::message_queue, params::Params};

pub(super) async fn mailbox_survives(agent_id: &str, params: &Params) {
    message_queue::enqueue(agent_id, "queued work".into(), params)
        .await
        .unwrap();
    message_queue::enqueue(agent_id, "second".into(), params)
        .await
        .unwrap();
    let (receipt, message) = message_queue::claim_for_test(agent_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(message, "queued work");
    message_queue::reset_for_test().await;
    assert!(message_queue::hydrate(agent_id).await.unwrap());
    assert_eq!(
        message_queue::claim_for_test(agent_id)
            .await
            .unwrap()
            .map(|item| item.0),
        Some(receipt.clone())
    );
    message_queue::ack_for_test(agent_id, &receipt)
        .await
        .unwrap();
    assert_eq!(
        message_queue::claim_for_test(agent_id)
            .await
            .unwrap()
            .unwrap()
            .1,
        "second"
    );
}
