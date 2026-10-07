use super::load_or_create_at;

#[test]
fn persists_one_identity_per_workspace() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("a2a/agent-identity");
    let first = load_or_create_at(&path).expect("create identity");
    let second = load_or_create_at(&path).expect("load identity");

    assert_eq!(first, second);
    assert!(first.starts_with("ctagent_"));
}

#[test]
fn concurrent_creators_share_one_identity() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("a2a/agent-identity");
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let path = path.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                load_or_create_at(&path).expect("create shared identity")
            })
        })
        .collect();
    let identities: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();

    assert!(identities.iter().all(|identity| identity == &identities[0]));
    let files = std::fs::read_dir(path.parent().unwrap())
        .expect("read identity directory")
        .count();
    assert_eq!(files, 1);
}
