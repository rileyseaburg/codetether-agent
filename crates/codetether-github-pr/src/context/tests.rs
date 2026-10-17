use super::parse_github_remote;

#[test]
fn parses_https_remote() {
    let (owner, repo) = parse_github_remote("https://github.com/a/b.git").unwrap();
    assert_eq!((owner.as_str(), repo.as_str()), ("a", "b"));
}

#[test]
fn parses_ssh_remote() {
    let (owner, repo) = parse_github_remote("git@github.com:a/b.git").unwrap();
    assert_eq!((owner.as_str(), repo.as_str()), ("a", "b"));
}
