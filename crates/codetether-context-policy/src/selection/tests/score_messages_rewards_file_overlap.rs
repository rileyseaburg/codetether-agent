//! Existing context selection regression.
use super::*;

#[test]
fn score_messages_rewards_file_overlap() {
    let msgs = vec![
        assistant("looking at src/lib.rs"), // overlap
        assistant("totally unrelated noise"),
        user("touch src/lib.rs"),
    ];
    let task = task_signature(&msgs);
    let scores = score_messages(&msgs, &task);
    // The earlier overlapping message scores higher than the noise.
    assert!(scores[0] > scores[1]);
}
