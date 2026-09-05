//! Equivalent histories must become ancestors before safe branch cleanup.
use super::super::{git, merge, outcome::Outcome};

#[test]
fn equivalent_merge_records_ancestry_without_changing_the_tree() {
    let root = tempfile::tempdir().unwrap();
    let repo = root.path();
    for args in [
        vec!["init", "-b", "main"],
        vec!["config", "user.name", "Test"],
        vec!["config", "user.email", "test@example.com"],
    ] {
        git::text(repo, &args).unwrap();
    }
    for (branch, content, message) in [
        ("", "base\n", "base"),
        ("feature", "same\n", "feature change"),
        ("main", "same\n", "main change"),
    ] {
        if branch == "feature" {
            git::text(repo, &["checkout", "-b", branch]).unwrap();
        } else if branch == "main" {
            git::text(repo, &["checkout", branch]).unwrap();
        }
        std::fs::write(repo.join("file"), content).unwrap();
        git::text(repo, &["add", "file"]).unwrap();
        git::text(repo, &["commit", "--no-gpg-sign", "-m", message]).unwrap();
    }
    let tree = git::text(repo, &["rev-parse", "HEAD^{tree}"]).unwrap();
    assert!(
        !git::output(repo, &["branch", "-d", "feature"])
            .unwrap()
            .status
            .success()
    );
    let Outcome::Merged(files) = merge::merge(repo, "feature").unwrap() else {
        panic!("equivalent histories should merge");
    };
    assert!(files.is_empty());
    assert_eq!(
        tree,
        git::text(repo, &["rev-parse", "HEAD^{tree}"]).unwrap()
    );
    git::text(repo, &["merge-base", "--is-ancestor", "feature", "HEAD"]).unwrap();
    let head = git::text(repo, &["rev-parse", "HEAD"]).unwrap();
    // An already-integrated branch must not create another merge commit.
    merge::merge(repo, "feature").unwrap();
    assert_eq!(head, git::text(repo, &["rev-parse", "HEAD"]).unwrap());
    git::text(repo, &["branch", "-d", "feature"]).unwrap();
}
