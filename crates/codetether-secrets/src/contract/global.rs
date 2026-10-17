//! Global initialization is verified in a subprocess to avoid polluting other tests.
#[test]
fn global_facade_contract() {
    if std::env::var_os("CODETETHER_TEST_GLOBAL_SECRETS").is_some() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            assert!(crate::secrets_manager().is_none());
            assert!(crate::get_api_key("fixture").await.is_none());
            let manager = crate::SecretsManager::default();
            manager
                .cache
                .write()
                .await
                .insert("fixture".into(), "fixture-key".into());
            crate::init_from_manager(manager).unwrap();
            assert!(crate::has_api_key("fixture").await);
            assert!(crate::get_api_key("fixture").await.as_deref() == Some("fixture-key"));
            assert!(crate::init_from_manager(crate::SecretsManager::default()).is_err());
            assert!(crate::verify_reachable().await.is_err());
            assert!(
                crate::set_provider_secrets("fixture", &Default::default())
                    .await
                    .is_err()
            );
        });
        return;
    }
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "contract::global::global_facade_contract"])
        .env("CODETETHER_TEST_GLOBAL_SECRETS", "1")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
