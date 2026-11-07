//! Pinned AppRole endpoint and file reference; deliberately no Debug output.
use anyhow::{Result, ensure};
use std::path::PathBuf;

pub(crate) struct AppRoleAuth {
    pub(super) address: String,
    pub(super) role_id: String,
    pub(super) login_path: String,
    pub(super) secret_id_file: PathBuf,
}

impl AppRoleAuth {
    pub(crate) fn new(
        address: &str,
        role_id: &str,
        mount: &str,
        secret_id_file: PathBuf,
    ) -> Result<Self> {
        ensure!(!role_id.trim().is_empty(), "AppRole role ID is empty");
        ensure!(
            secret_id_file.is_absolute(),
            "AppRole secret ID file must be absolute"
        );
        Ok(Self {
            address: crate::login::normalize(address)?,
            role_id: role_id.trim().into(),
            login_path: crate::login::http::auth_path(mount, "approle")?,
            secret_id_file,
        })
    }
}
