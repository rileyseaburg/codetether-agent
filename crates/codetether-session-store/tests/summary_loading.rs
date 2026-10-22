//! Regression tests for selecting usable summaries before applying the row cap.
use codetether_context_policy::summary_index::{Granularity, SummaryNode, SummaryRange};
use codetether_session_store::{schema, summaries};
use rusqlite::{Connection, params};

#[test]
fn out_of_window_summaries_do_not_hide_valid_older_entries() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(schema::SQL).unwrap();
    let messages = vec![
        codetether_provider_core::Message {
            role: codetether_provider_core::Role::User,
            content: vec![]
        };
        10
    ];
    let node = serde_json::to_string(&SummaryNode {
        content: "retained context".into(),
        target_tokens: 64,
        granularity: Granularity::Turn,
        generation: 0,
        provenance: Some(
            codetether_context_policy::summary_index::SummaryProvenance::new(
                &messages,
                "fixture",
                Some("original-model"),
            )
            .unwrap(),
        ),
    })
    .unwrap();
    db.execute(
        "INSERT INTO projections VALUES ('s','manual_summary',10,20,?1)",
        [&node],
    )
    .unwrap();
    for start in 21..150 {
        db.execute(
            "INSERT INTO projections VALUES ('s','manual_summary',?1,300,?2)",
            params![start, node],
        )
        .unwrap();
    }
    // Invalid empty/reversed ranges also must not occupy the limited result set.
    for start in 150..280 {
        db.execute(
            "INSERT INTO projections VALUES ('s','manual_summary',?1,?1,?2)",
            params![start, node],
        )
        .unwrap();
    }
    let index = summaries::read(&db, "s", 10, 280).unwrap();
    assert_eq!(index.len(), 1);
    assert!(
        index
            .get(SummaryRange::new(0, 10).unwrap())
            .unwrap()
            .valid_for(&messages, SummaryRange::new(0, 10).unwrap())
    );
    let mut altered = messages;
    altered[5].role = codetether_provider_core::Role::Assistant;
    let mut projection = index.clone();
    assert_eq!(projection.retain_valid(&altered), 1);
    assert_eq!(
        index.len(),
        1,
        "persisted entry remains available for audit"
    );
    assert_eq!(
        index
            .get(SummaryRange::new(0, 10).unwrap())
            .unwrap()
            .content,
        "retained context"
    );
    assert!(summaries::read(&db, "another", 10, 280).unwrap().is_empty());
    assert!(summaries::read(&db, "s", 11, 280).unwrap().is_empty());
}
