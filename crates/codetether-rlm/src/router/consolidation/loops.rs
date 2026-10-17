//! Loop exits, errors, limits, and model selection.
use super::{fixture::run, provider::text};
#[tokio::test]
async fn loop_transitions_snapshot() {
    let mut results = Vec::new();
    results.push(
        run(
            vec![text("working")],
            vec![text("FINAL(\"done\")")],
            5,
            5,
            false,
            false,
        )
        .await,
    );
    results.push(run(vec![Err("first failed".into())], vec![], 5, 5, false, false).await);
    results.push(
        run(
            vec![text("working")],
            vec![Err("later failed".into())],
            5,
            5,
            false,
            false,
        )
        .await,
    );
    results.push(run(vec![text("working")], vec![], 5, 1, false, false).await);
    results.push(
        run(
            vec![text("working")],
            vec![text("working")],
            2,
            5,
            false,
            false,
        )
        .await,
    );
    results.push(run(vec![], vec![], 5, 5, false, true).await);
    results.push(run(vec![text("plain summary")], vec![], 5, 5, true, false).await);
    let tools = super::tool_reply::response();
    results.push(run(vec![Ok(tools)], vec![], 5, 5, false, false).await);
    super::assertions::check(&results);
    results.push(run(vec![], vec![], 0, 5, false, false).await);
    results.push(run(vec![text("working")], vec![], 5, 0, false, false).await);
    println!("LOOP_CASES={}", serde_json::to_string(&results).unwrap());
}
