//! Mocked HTTP tests for privilege rejection and credential-safe failures.

#[tokio::test]
async fn app_scoped_tokens_are_accepted() {
    let server = super::http_fixture::start("codetether-client", "deny", false).await;
    let facts = super::validate::token(&server.address, "fixture-token")
        .await
        .unwrap();
    assert_eq!(facts.policies, ["codetether-client"]);
    assert_eq!(facts.ttl, 3600);
}

#[tokio::test]
async fn admin_policies_and_hidden_admin_capabilities_are_rejected() {
    for (policy, capability) in [
        ("root", "deny"),
        ("custom-policy", "sudo"),
        ("custom-policy", "update"),
    ] {
        let server = super::http_fixture::start(policy, capability, false).await;
        assert!(
            super::validate::token(&server.address, "fixture-token")
                .await
                .is_err()
        );
    }
}

#[tokio::test]
async fn vault_rejection_does_not_echo_response_token() {
    let server = super::http_fixture::start("default", "deny", true).await;
    let error = super::validate::token(&server.address, "fixture-token")
        .await
        .err()
        .unwrap();
    assert!(error.to_string().contains("403"));
    assert!(!format!("{error:#}").contains("fixture-token"));
}
