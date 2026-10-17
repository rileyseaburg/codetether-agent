//! Manager configured only for the local fixture.
use super::Fixture;
impl Fixture {
    pub async fn manager(&self) -> crate::SecretsManager {
        crate::SecretsManager::new(&crate::VaultConfig {
            address: self.address.clone(),
            token: "fixture-token".into(),
            mount: None,
            path: None,
        })
        .await
        .unwrap()
    }
}
