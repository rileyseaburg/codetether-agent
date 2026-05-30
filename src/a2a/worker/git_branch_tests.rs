//! Tests for worker Git branch helpers.

use super::git_refspec::push_refspec;

#[test]
fn push_refspec_uses_full_heads_namespace_for_slash_branches() {
    assert_eq!(
        push_refspec("codetether/issue-587").expect("valid branch"),
        "HEAD:refs/heads/codetether/issue-587"
    );
}

#[test]
fn push_refspec_rejects_invalid_branch_names() {
    for branch in ["HEAD", "bad:branch", "bad..branch", "../bad"] {
        assert!(push_refspec(branch).is_err());
    }
}
