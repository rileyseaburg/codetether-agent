use crate::tui::symbol_search::SymbolEntry;

pub fn apply(results: &mut [SymbolEntry], query: &str) {
    let query = query.to_lowercase();
    results.sort_by_key(|symbol| score(&symbol.name.to_lowercase(), &query));
}

fn score(name: &str, query: &str) -> (u8, usize, String) {
    let tier = if name == query {
        0
    } else if name.starts_with(query) {
        1
    } else if name.contains(query) {
        2
    } else {
        3
    };
    (tier, name.len(), name.to_string())
}

#[cfg(test)]
mod tests {
    use super::apply;
    use crate::tui::symbol_search::SymbolEntry;
    use std::path::PathBuf;

    #[test]
    fn exact_and_prefix_matches_rank_first() {
        let mut values = [entry("captureError"), entry("recapture"), entry("capture")];
        apply(&mut values, "capture");
        assert_eq!(
            values.map(|value| value.name),
            ["capture", "captureError", "recapture"]
        );
    }

    fn entry(name: &str) -> SymbolEntry {
        SymbolEntry {
            name: name.into(),
            kind: "Function".into(),
            path: PathBuf::new(),
            uri: None,
            line: None,
            container: None,
        }
    }
}
