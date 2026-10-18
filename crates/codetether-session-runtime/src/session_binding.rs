//! Frontend session identity and retained-view projections.

/// Bind a frontend session's identifier to shared runtime requests.
///
/// # Examples
/// ```text
/// codetether_session_runtime::impl_frontend_session_identity!(Session);
/// ```
#[macro_export]
macro_rules! impl_frontend_session_identity {
    ($session:ty $(,)?) => {
        impl $crate::request::SessionIdentity for $session {
            fn session_id(&self) -> &str {
                &self.id
            }
        }
    };
}

/// Bind the frontend's read-only session snapshot to a shared session slot.
///
/// Fields retain the selected session's metadata and message count. Principal
/// derivation uses the existing frontend provenance constructor.
///
/// # Examples
/// ```text
/// codetether_session_runtime::impl_frontend_session_view!(
///     Session, SessionView, RuntimePrincipal,
/// );
/// ```
#[macro_export]
macro_rules! impl_frontend_session_view {
    ($session:ty, $view:path, $principal:path $(,)?) => {
        use $principal as RuntimePrincipal;
        use $view as SessionView;
        impl $crate::slot::ViewSource for $session {
            type View = SessionView;
            fn snapshot(&self) -> SessionView {
                SessionView {
                    id: self.id.clone(),
                    title: self.title.clone(),
                    model: self.metadata.model.clone(),
                    directory: self.metadata.directory.clone(),
                    auto_apply_edits: self.metadata.auto_apply_edits,
                    allow_network: self.metadata.allow_network,
                    slash_autocomplete: self.metadata.slash_autocomplete,
                    use_worktree: self.metadata.use_worktree,
                    message_count: self.messages.len(),
                    principal: RuntimePrincipal::for_session(
                        &self.agent,
                        self.metadata.provenance.as_ref(),
                    ),
                }
            }
        }
    };
}
