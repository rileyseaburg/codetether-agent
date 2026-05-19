use super::config::{BudgetRule, GuardConfig};

pub fn load() -> GuardConfig {
    GuardConfig {
        rules: specs()
            .into_iter()
            .filter_map(|spec| spec.split_once(':'))
            .filter_map(|(glob, max)| BudgetRule::new(glob, max.parse().ok()?))
            .collect(),
    }
}

fn specs() -> [&'static str; 5] {
    [
        "src/**/*.rs:50",
        "**/*.ts:80",
        "**/*.tsx:80",
        "**/*.js:80",
        "**/*.jsx:80",
    ]
}
