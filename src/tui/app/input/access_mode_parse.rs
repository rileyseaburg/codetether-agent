use crate::config::AccessMode;

pub(super) fn prompt(prompt: &str) -> Option<(&'static str, &str)> {
    commands().into_iter().find_map(|cmd| {
        crate::tui::app::text::command_with_optional_args(prompt, cmd).map(|rest| (cmd, rest))
    })
}

pub(super) fn mode(value: &str) -> Option<AccessMode> {
    match value.to_ascii_lowercase().as_str() {
        "ask" | "read-only" => Some(AccessMode::Ask),
        "approve" | "workspace-write" | "sandbox" => Some(AccessMode::Approve),
        "full" | "danger-full-access" => Some(AccessMode::Full),
        _ => None,
    }
}

pub(super) fn label(mode: AccessMode) -> &'static str {
    match mode {
        AccessMode::Ask => "ask",
        AccessMode::Approve => "approve",
        AccessMode::Full => "full",
    }
}

fn commands() -> [&'static str; 4] {
    ["/access-mode", "/access", "/sandbox-mode", "/permissions"]
}

#[cfg(test)]
mod tests {
    use super::{mode, prompt};
    use crate::config::AccessMode;

    #[test]
    fn parses_access_mode_aliases() {
        assert_eq!(mode("read-only"), Some(AccessMode::Ask));
        assert_eq!(mode("workspace-write"), Some(AccessMode::Approve));
        assert_eq!(mode("danger-full-access"), Some(AccessMode::Full));
    }

    #[test]
    fn parses_permissions_set_alias() {
        assert_eq!(prompt("/permissions full"), Some(("/permissions", "full")));
        assert_eq!(
            prompt("/sandbox-mode danger-full-access"),
            Some(("/sandbox-mode", "danger-full-access"))
        );
    }
}
