use serde::{Deserialize, Serialize};

/// Permission profile selector accepted as a string or structured table.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PermissionProfileConfig {
    /// Compact TOML form, for example `permission_profile = "disabled"`.
    Named(PermissionProfile),
    /// Structured TOML form, for example `[permission_profile] type = "disabled"`.
    Detailed(PermissionProfileDetails),
}

impl PermissionProfileConfig {
    /// Return the selected profile kind.
    pub fn profile_type(&self) -> PermissionProfile {
        match self {
            Self::Named(profile) => *profile,
            Self::Detailed(details) => details.profile_type,
        }
    }
}

/// Structured permission profile details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PermissionProfileDetails {
    /// Broad permission profile type.
    #[serde(rename = "type", default)]
    pub profile_type: PermissionProfile,
}

/// Broad permission profile for future runtime permission gates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionProfile {
    /// Conservative profile for existing configs with no explicit profile.
    #[default]
    Conservative,
    /// Codex built-in managed read-only profile.
    #[serde(rename = ":read-only", alias = "read-only")]
    ReadOnly,
    /// Codex built-in managed workspace-write profile.
    #[serde(rename = ":workspace", alias = "workspace")]
    Workspace,
    /// Codex built-in profile with sandboxing disabled.
    #[serde(rename = ":danger-full-access", alias = "danger-full-access")]
    DangerFullAccess,
    /// Disable the permission profile layer.
    Disabled,
}
