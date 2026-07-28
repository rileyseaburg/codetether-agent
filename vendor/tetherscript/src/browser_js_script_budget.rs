const BASE_BUDGET: u64 = 20_000_000;
const BUDGET_PER_BYTE: u64 = 80;
const MAX_BUDGET: u64 = 1_000_000_000;

pub(crate) fn for_source(source: &str) -> u64 {
    BASE_BUDGET
        .saturating_add((source.len() as u64).saturating_mul(BUDGET_PER_BYTE))
        .min(MAX_BUDGET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_budget_scales_for_large_production_bundles() {
        assert_eq!(for_source("let x=1;"), BASE_BUDGET + 640);
        assert_eq!(for_source(&"x".repeat(20_000_000)), MAX_BUDGET);
    }
}
