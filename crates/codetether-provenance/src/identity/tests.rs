//! Git author formatting regressions.

use super::{git_author_email, git_author_name};

#[test]
fn derives_unique_email_from_agent_identity() {
    let email = git_author_email(Some("user:abc-123"), None, None);
    assert_eq!(email, "agent+user-abc-123@codetether.run");
}

#[test]
fn includes_identity_in_display_name() {
    let name = git_author_name(Some("user:abc-123"));
    assert_eq!(name, "CodeTether Agent [user-abc-123]");
}
