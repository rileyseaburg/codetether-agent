use super::{list, test_support};
use crate::tool::agent::{collaboration_runtime::thread_status, store};

#[tokio::test]
async fn lists_one_root_tree_and_resolves_relative_prefixes() {
    let root = format!("root-{}", uuid::Uuid::new_v4());
    let other_root = format!("root-{}", uuid::Uuid::new_v4());
    let parent = test_support::entry("researcher", &root).await;
    let parent_id = parent.id().to_string();
    let worker = test_support::entry("worker", &parent_id).await;
    let worker_id = worker.id().to_string();
    let unrelated = test_support::entry("other", &other_root).await;
    let unrelated_id = unrelated.id().to_string();
    for entry in [parent, worker, unrelated] {
        store::insert(entry);
    }
    thread_status::set(
        &parent_id,
        thread_status::ThreadStatus::Completed(Some("done".into())),
    );
    thread_status::set(&worker_id, thread_status::ThreadStatus::Interrupted);

    let agents = list(&root, None).unwrap();
    assert_eq!(
        test_support::names(&agents),
        ["/root", "/root/researcher", "/root/researcher/worker"]
    );
    assert!(matches!(
        agents[1].agent_status,
        thread_status::ThreadStatus::Completed(_)
    ));
    let filtered = list(&parent_id, Some("worker")).unwrap();
    assert_eq!(test_support::names(&filtered), ["/root/researcher/worker"]);

    for id in [&parent_id, &worker_id, &unrelated_id] {
        store::remove(id);
        thread_status::remove(id);
    }
}
