use crate::tool::ToolResult;

#[test]
fn remote_turn_is_visible_only_to_its_parent() {
    let name = format!("peer-{}", uuid::Uuid::new_v4());
    let turn = super::begin(&name, Some("parent-a"), "inspect CI");
    let running = super::snapshots(Some("parent-a"));
    assert!(
        running
            .iter()
            .any(|peer| peer.name == name && peer.is_processing)
    );
    assert!(super::snapshots(Some("parent-b")).is_empty());

    turn.complete(&ToolResult::success("run failed in lint"));
    let done = super::snapshots(Some("parent-a"));
    assert!(
        done.iter()
            .any(|peer| peer.name == name && !peer.is_processing)
    );
    let transcript = super::transcript(&name, "parent-a").unwrap();
    assert_eq!(transcript.len(), 2);
}

#[test]
fn cancelled_remote_turn_does_not_remain_working() {
    let suffix = uuid::Uuid::new_v4();
    let name = format!("cancelled-peer-{suffix}");
    let parent = format!("cancelled-parent-{suffix}");
    let turn = super::begin(&name, Some(&parent), "wait forever");
    drop(turn);

    let peers = super::snapshots(Some(&parent));
    let peer = peers.iter().find(|peer| peer.name == name).unwrap();
    assert!(!peer.is_processing);
    assert!(peer.failed);
    assert_eq!(super::transcript(&name, &parent).unwrap().len(), 2);
}
