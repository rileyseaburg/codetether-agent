use crate::config::Config;

impl Config {
    pub(super) fn merge(mut self, other: Self) -> Self {
        self.merge_defaults(&other);
        if other.a2a.server_url.is_some() {
            self.a2a = other.a2a.clone();
        }
        self.providers.extend(other.providers);
        self.agents.extend(other.agents);
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
