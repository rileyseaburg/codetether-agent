//! Mocked AppRole recovery: no human prompt or live Vault credentials.
use super::{fixture::Fixture, response};
use crate::{SecretsManager, approle::AppRoleAuth};
mod denial;
mod recovery;
mod validation;

async fn manager(server: &Fixture, file: &std::path::Path) -> SecretsManager {
    SecretsManager::from_approle(
        AppRoleAuth::new(&server.address, "fixture-role", "approle", file.into()).unwrap(),
        None,
        None,
    )
    .await
    .unwrap()
}

fn secret_file() -> tempfile::NamedTempFile {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), "fixture-secret\n").unwrap();
    file
}
