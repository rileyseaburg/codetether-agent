//! Tests for Git credential helper utilities.
//!
//! These tests cover GitHub CLI delegation and query rendering without making
//! network calls to the control plane.
//!
//! # Examples
//!
//! ```ignore
//! cargo test git_credentials
//! ```

use super::gh_cli::should_delegate_to_gh_cli;
use super::gh_query::render_gh_credential_query;
use super::{GitCredentialMaterial, GitCredentialQuery};

fn sample_credentials() -> GitCredentialMaterial {
    GitCredentialMaterial {
        username: "x-access-token".to_string(),
        password: "secret".to_string(),
        expires_at: None,
        token_type: "github_app".to_string(),
        host: Some("github.com".to_string()),
        path: Some("owner/repo.git".to_string()),
    }
}

#[test]
fn delegates_to_gh_for_github_host() {
    let query = GitCredentialQuery {
        host: Some("github.com".to_string()),
        ..GitCredentialQuery::default()
    };
    assert!(should_delegate_to_gh_cli(&query, &sample_credentials()));
}

#[test]
fn skips_gh_for_non_github_host() {
    let query = GitCredentialQuery {
        host: Some("gitlab.com".to_string()),
        ..GitCredentialQuery::default()
    };
    assert!(!should_delegate_to_gh_cli(&query, &sample_credentials()));
}

#[test]
fn renders_git_credential_payload_for_gh() {
    let query = GitCredentialQuery {
        protocol: Some("https".to_string()),
        ..GitCredentialQuery::default()
    };
    let rendered = render_gh_credential_query(&query, &sample_credentials());
    assert!(rendered.contains("protocol=https\n"));
    assert!(rendered.contains("host=github.com\n"));
    assert!(rendered.contains("path=owner/repo.git\n"));
    assert!(rendered.ends_with("\n\n"));
}
