//! Summary writes wait for competing writers before checking their source revision.
use super::super::{connection, save, summaries};
use super::message;
use crate::session::{
    Session,
    index::{Granularity, SummaryNode, SummaryRange},
};
use std::time::Duration;

#[tokio::test]
async fn summary_waits_for_an_existing_writer() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("summary.json");
    let mut session = Session::new().await.unwrap();
    session.messages.push(message("preserved"));
    save(&session, &path).await.unwrap();
    let range = SummaryRange::new(0, 1).unwrap();
    let node = SummaryNode {
        content: "preserved summary".into(),
        target_tokens: 32,
        granularity: Granularity::Turn,
        generation: 1,
    };
    let writer = connection::open(&path).unwrap();
    writer.execute_batch("BEGIN IMMEDIATE").unwrap();
    let expected = node.clone();
    let mut commit = tokio::spawn(async move { summaries::put(&session, range, &node).await });
    let early = tokio::time::timeout(Duration::from_millis(150), &mut commit).await;
    writer.execute_batch("COMMIT").unwrap();
    assert!(early.is_err(), "summary did not wait for writer: {early:?}");
    commit.await.unwrap().unwrap();
    let body: String = writer
        .query_row(
            "SELECT body FROM projections WHERE name='manual_summary'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(serde_json::from_str::<SummaryNode>(&body).unwrap(), expected);
}

