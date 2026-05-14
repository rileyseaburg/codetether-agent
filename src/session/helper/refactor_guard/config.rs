use serde::Deserialize;
use std::path::Path;

pub(super) struct GuardConfig {
    pub rules: Vec<BudgetRule>,
}

pub(super) struct BudgetRule {
    pub pattern: glob::Pattern,
    pub max_lines: usize,
}

#[derive(Deserialize)]
struct FileConfig {
    #[serde(default)]
    rules: Vec<FileRule>,
}

#[derive(Deserialize)]
struct FileRule {
    glob: String,
    max_lines: usize,
}

impl GuardConfig {
    pub fn load(root: &Path) -> Self {
        let path = root.join(".codetether/refactor_guard.toml");
        let Some(raw) = std::fs::read_to_string(path).ok() else {
            return super::config_defaults::load();
        };
        let Ok(file) = toml::from_str::<FileConfig>(&raw) else {
            return super::config_defaults::load();
        };
        let rules = file
            .rules
            .into_iter()
            .filter_map(|rule| BudgetRule::new(&rule.glob, rule.max_lines))
            .collect();
        Self { rules }
    }

    pub fn limit_for(&self, path: &str) -> Option<usize> {
        self.rules
            .iter()
            .find(|rule| rule.pattern.matches(path))
            .map(|rule| rule.max_lines)
    }
}

impl BudgetRule {
    pub fn new(pattern: &str, max_lines: usize) -> Option<Self> {
        Some(Self {
            pattern: glob::Pattern::new(pattern).ok()?,
            max_lines,
        })
    }
}
