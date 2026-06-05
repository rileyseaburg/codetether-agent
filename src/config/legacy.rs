use crate::config::Config;

impl Config {
    pub(super) fn normalize_legacy_defaults(&mut self) {
        self.normalize_legacy_provider();
        if self.has_legacy_default_model() {
            self.default_model = Some("minimax/MiniMax-M3".to_string());
            if self.should_migrate_provider() {
                self.default_provider = Some("minimax".to_string());
            }
        }
    }

    fn normalize_legacy_provider(&mut self) {
        if let Some(provider) = self.default_provider.as_deref()
            && provider.trim().eq_ignore_ascii_case("zhipuai")
        {
            self.default_provider = Some("zai".to_string());
        }
    }

    fn has_legacy_default_model(&self) -> bool {
        self.default_model.as_deref().is_some_and(|model| {
            let m = model.trim();
            m.eq_ignore_ascii_case("moonshotai/kimi-k2.5")
                || m.eq_ignore_ascii_case("kimi-k2.5")
                || m.eq_ignore_ascii_case("zai/glm-5")
                || m.eq_ignore_ascii_case("zhipuai/glm-5")
        })
    }

    fn should_migrate_provider(&self) -> bool {
        self.default_provider.as_deref().is_none_or(|p| {
            let p = p.trim();
            p.eq_ignore_ascii_case("moonshotai")
                || p.eq_ignore_ascii_case("zhipuai")
                || p.eq_ignore_ascii_case("zai")
        })
    }
}
