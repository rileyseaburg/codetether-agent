impl<S: super::SessionIdentity, R, I> std::fmt::Debug for super::PromptRequest<S, R, I> {
    /// Formats a prompt request without dumping heavyweight runtime state.
    ///
    /// The debug representation includes the session identifier, prompt text,
    /// and attachment count, but omits the provider registry, directory, and
    /// worktree details to keep logs readable and avoid exposing unnecessary
    /// execution state.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PromptRequest")
            .field("session_id", &self.session.session_id())
            .field("prompt", &self.prompt)
            .field("image_count", &self.images.len())
            .finish_non_exhaustive()
    }
}
