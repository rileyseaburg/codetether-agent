use crate::config::Config;

impl Config {
    pub(super) fn merge(mut self, other: Self) -> Self {
        self.merge_defaults(&other);
        self.merge_codex_policy(&other);
        if other.a2a.server_url.is_some() {
            self.a2a = other.a2a.clone();
        }
        self.providers.extend(other.providers);
        if other.agents.interrupt_message.is_some() {
            self.agents.interrupt_message = other.agents.interrupt_message;
        }
        if other.agents.max_concurrent_threads_per_session.is_some() {
            self.agents.max_concurrent_threads_per_session =
                other.agents.max_concurrent_threads_per_session;
        }
        if other.agents.max_depth.is_some() {
            self.agents.max_depth = other.agents.max_depth;
        }
        self.agents.profiles.extend(other.agents.profiles);
        self.merge_permissions(other.permissions);
        self.merge_telemetry(other.telemetry);
        self.merge_lsp(other.lsp);
        self
    }

    fn merge_defaults(&mut self, other: &Self) {
        if other.default_provider.is_some() {
            self.default_provider.clone_from(&other.default_provider);
        }
        if other.default_model.is_some() {
            self.default_model.clone_from(&other.default_model);
        }
    }
}
