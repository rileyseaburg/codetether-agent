//! Workspace-aware descriptions for automatically started A2A peers.

pub(super) fn resolve(explicit: Option<&str>) -> String {
    explicit.map(ToString::to_string).unwrap_or_else(|| {
        let workspace = std::env::current_dir()
            .ok()
            .and_then(|path| path.file_name().map(|name| name.to_owned()))
            .and_then(|name| name.into_string().ok())
            .unwrap_or_else(|| "workspace".to_string());
        format!("CodeTether agent for workspace {workspace}")
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn preserves_an_explicit_description() {
        assert_eq!(super::resolve(Some("Voice API owner")), "Voice API owner");
    }
}
