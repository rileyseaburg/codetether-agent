use super::sanitize_project_overlay;
use crate::config::{AccessMode, Config};

#[test]
fn untrusted_project_overlay_cannot_elevate_access_mode() {
    let overlay: Config = toml::from_str(r#"access_mode = "full""#).unwrap();

    let merged = Config::default().merge(sanitize_project_overlay(&Config::default(), overlay));

    assert_eq!(merged.access_mode, None);
    assert_eq!(merged.effective_access_mode(), Some(AccessMode::Ask));
}

#[test]
fn untrusted_project_overlay_cannot_set_policy_requirements() {
    let overlay: Config = toml::from_str(
        r#"
[requirements]
allowed_access_modes = ["full"]
"#,
    )
    .unwrap();

    let merged = Config::default().merge(sanitize_project_overlay(&Config::default(), overlay));

    assert!(merged.requirements.is_empty());
}
