use super::{load, replay, tests::goal};
use codetether_goal_core::TaskLog;
use std::io::Write;

#[test]
fn appends_replay_only_new_bytes_and_equal_a_cold_fold() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("goal.jsonl"));
    log.append_blocking(&goal("first")).unwrap();
    let mut file = std::fs::File::open(log.path()).unwrap();
    let metadata = file.metadata().unwrap();
    let original = replay::read(&mut file, &metadata, None).unwrap();
    for index in 0..128 {
        log.append_blocking(&goal(&format!("goal {index}")))
            .unwrap();
    }
    let metadata = file.metadata().unwrap();
    let length = metadata.len();
    let appended = replay::read(&mut file, &metadata, Some(&original)).unwrap();
    assert_eq!(appended.replayed_bytes, length - original.length);
    let cold = replay::read(&mut file, &metadata, None).unwrap();
    assert_eq!(
        appended.state.goal.unwrap().objective,
        cold.state.goal.unwrap().objective
    );
}

#[test]
fn incomplete_or_corrupt_appends_do_not_erase_cached_governance() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("goal.jsonl"));
    log.append_blocking(&goal("preserve me")).unwrap();
    load(&log).unwrap();
    let length = std::fs::metadata(log.path()).unwrap().len();
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(log.path())
        .unwrap();
    file.write_all(b"{broken").unwrap();
    assert!(load(&log).unwrap_err().to_string().contains("incomplete"));
    file.write_all(b"}\n").unwrap();
    assert!(load(&log).unwrap_err().to_string().contains("invalid"));
    file.set_len(length).unwrap();
    assert_eq!(load(&log).unwrap().goal.unwrap().objective, "preserve me");
}

#[test]
fn replacement_and_deletion_invalidate_cached_state() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("goal.jsonl"));
    log.append_blocking(&goal("old")).unwrap();
    load(&log).unwrap();
    let replacement = TaskLog::at(dir.path().join("replacement"));
    replacement
        .append_blocking(&goal("a longer replacement objective"))
        .unwrap();
    std::fs::rename(replacement.path(), log.path()).unwrap();
    assert_eq!(
        load(&log).unwrap().goal.unwrap().objective,
        "a longer replacement objective"
    );
    std::fs::remove_file(log.path()).unwrap();
    assert!(load(&log).unwrap_err().to_string().contains("missing"));
}

#[test]
fn malformed_governance_is_visible_in_the_prompt() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("goal.jsonl"));
    std::fs::write(log.path(), b"not an event\n").unwrap();
    let prompt = crate::compose_from_log("Base instructions".into(), &log);
    assert!(prompt.starts_with("Base instructions"));
    assert!(prompt.contains("GOAL_GOVERNANCE_UNAVAILABLE"));
    assert!(prompt.contains("before continuing goal work"));
}

#[test]
fn recreated_path_does_not_reuse_a_cached_prefix() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("goal.jsonl"));
    log.append_blocking(&goal("old")).unwrap();
    assert_eq!(load(&log).unwrap().goal.unwrap().objective, "old");
    std::fs::remove_file(log.path()).unwrap();
    log.append_blocking(&goal("replacement with a longer objective"))
        .unwrap();
    assert_eq!(
        load(&log).unwrap().goal.unwrap().objective,
        "replacement with a longer objective"
    );
}

#[cfg(unix)]
#[test]
fn replacement_between_open_and_replay_is_a_coherent_snapshot() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("goal.jsonl"));
    log.append_blocking(&goal("old snapshot")).unwrap();
    let mut old_file = std::fs::File::open(log.path()).unwrap();
    let old_metadata = old_file.metadata().unwrap();
    let replacement = TaskLog::at(dir.path().join("new.jsonl"));
    replacement
        .append_blocking(&goal("new snapshot with different size"))
        .unwrap();
    std::fs::rename(replacement.path(), log.path()).unwrap();
    let old = replay::read(&mut old_file, &old_metadata, None).unwrap();
    assert_eq!(old.state.goal.unwrap().objective, "old snapshot");
    assert_eq!(
        load(&log).unwrap().goal.unwrap().objective,
        "new snapshot with different size"
    );
}

#[cfg(unix)]
#[test]
fn concurrent_atomic_replacements_do_not_mix_goal_snapshots() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("goal.jsonl");
    let log = TaskLog::at(&path);
    log.append_blocking(&goal("goal 0")).unwrap();
    load(&log).unwrap();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for index in 1..64 {
                let staged = TaskLog::at(dir.path().join("staged.jsonl"));
                staged
                    .append_blocking(&goal(&format!("goal {index}")))
                    .unwrap();
                std::fs::rename(staged.path(), &path).unwrap();
            }
        });
        for _ in 0..128 {
            let goal = load(&log).unwrap().goal.unwrap();
            assert_eq!(goal.id, goal.objective);
            assert!(goal.objective.starts_with("goal "));
        }
    });
    assert_eq!(load(&log).unwrap().goal.unwrap().objective, "goal 63");
}
