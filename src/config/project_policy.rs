//! Trust gate for project-local configuration overlays.

use super::{Config, PermissionConfig, ProjectTrustLevel};

pub(super) fn apply_trust_store(mut config: Config) -> Config {
    if config.is_project_trusted() || !trust_store_trusts_current_workspace() {
        return config;
    }
    config.trust_level = Some(ProjectTrustLevel::Trusted);
    config
}

pub(super) fn sanitize_project_overlay(base: &Config, mut overlay: Config) -> Config {
    if base.is_project_trusted() {
        return overlay;
    }
    strip_policy_fields(&mut overlay);
    overlay
}

fn strip_policy_fields(config: &mut Config) {
    config.access_mode = None;
    config.sandbox_mode = None;
    config.approval_policy = None;
    config.trust_level = None;
    config.permission_profile = None;
    config.requirements = Default::default();
    config.permissions = PermissionConfig::default();
    config.providers.clear();
    config.agents.clear();
    config.a2a = Default::default();
    config.lsp = Default::default();
}

fn trust_store_trusts_current_workspace() -> bool {
    super::trust_store::ProjectTrustStore::for_current_workspace()
        .is_ok_and(|store| store.is_trusted())
}

#[cfg(test)]
#[path = "project_access_mode_tests.rs"]
mod access_mode_tests;
#[cfg(test)]
#[path = "project_policy_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "project_policy_trusted_tests.rs"]
mod trusted_tests;
