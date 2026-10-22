mod support;
use codetether_agent_replay::execute;
use support::{TestHost, call, session};

#[tokio::test]
async fn mixed_batch_persists_prior_results_and_uses_refreshed_state_for_later_calls() {
    let mut session = session();
    execute(
        &TestHost,
        &mut session,
        vec![
            call("a", "read"),
            call("b", "context_pin"),
            call("c", "read"),
        ],
    )
    .await
    .unwrap();
    assert_eq!(session.messages.len(), 4);
    assert_eq!(
        session
            .tools
            .iter()
            .map(|tool| tool.id.as_str())
            .collect::<Vec<_>>(),
        ["a", "b", "c"]
    );
    let disk = session.disk.lock().unwrap();
    assert_eq!(
        disk.messages.len(),
        2,
        "first result was saved before the external edit"
    );
    assert_eq!(
        disk.events,
        [
            "run:read:false",
            "record",
            "save",
            "run:context_pin:false",
            "refresh",
            "record",
            "run:read:true",
            "record"
        ]
    );
}

#[tokio::test]
async fn failed_save_prevents_execution_and_failed_refresh_retains_the_completed_result() {
    let mut before = session();
    before.fail_save = true;
    assert!(
        execute(&TestHost, &mut before, vec![call("a", "context_pin")])
            .await
            .is_err()
    );
    assert!(before.disk.lock().unwrap().events.is_empty());
    let mut after = session();
    after.fail_refresh = true;
    assert!(
        execute(
            &TestHost,
            &mut after,
            vec![call("a", "context_pin"), call("b", "read")]
        )
        .await
        .is_err()
    );
    assert_eq!(after.tools.len(), 1);
    assert!(after.tools[0].success);
    assert_eq!(after.messages.len(), 2);
    assert!(after.tools[0].output.contains("result:context_pin"));
}

#[tokio::test]
async fn read_only_and_denied_batches_do_not_create_context_writes() {
    let mut read = session();
    execute(
        &TestHost,
        &mut read,
        vec![call("a", "read"), call("b", "grep")],
    )
    .await
    .unwrap();
    assert_eq!(read.tools.len(), 2);
    assert!(
        !read
            .disk
            .lock()
            .unwrap()
            .events
            .iter()
            .any(|event| event == "save" || event == "refresh")
    );
    let mut denied = session();
    denied.deny = true;
    execute(&TestHost, &mut denied, vec![call("a", "context_pin")])
        .await
        .unwrap();
    assert_eq!(denied.disk.lock().unwrap().events, ["record"]);
    assert!(!denied.tools[0].success);
}
