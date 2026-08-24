//! Execution-state scope for reusable command-prefix grants.

pub(crate) fn from_args(args: &serde_json::Value) -> Option<String> {
    let workspace = super::super::workspace::from_args(args)?;
    let workspace = workspace.canonicalize().unwrap_or_else(|_| {
        workspace
            .is_absolute()
            .then_some(workspace.clone())
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default().join(&workspace))
    });
    Some(format!(
        "{}::network={}",
        workspace.display(),
        crate::tool::network_access::allowed_for(args)
    ))
}
