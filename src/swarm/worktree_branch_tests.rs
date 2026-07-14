use super::{fixture::Fixture, has_net_changes};

#[tokio::test]
async fn rejects_empty_or_reverted_branch_history() {
    let fixture = Fixture::new();
    fixture.checkout("agent");
    fixture.empty_commit("empty");
    fixture.write_commit("changed\n", "change");
    fixture.write_commit("base\n", "revert");
    fixture.checkout("main");
    assert!(
        !has_net_changes(&fixture.manager, &fixture.info)
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn recognizes_patch_already_integrated_elsewhere() {
    let fixture = Fixture::new();
    fixture.checkout("agent");
    fixture.write_commit("same\n", "agent change");
    fixture.checkout("main");
    fixture.write_commit("same\n", "main change");
    assert!(
        has_net_changes(&fixture.manager, &fixture.info)
            .await
            .unwrap()
    );
}
