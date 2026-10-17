//! Select file references for fallback context.

pub(super) fn collect_file_lines<'a>(lines: &'a [&'a str]) -> Vec<&'a str> {
    lines
        .iter()
        .filter_map(|l| {
            (l.contains(".ts") || l.contains(".rs") || l.contains(".py") || l.contains(".json"))
                .then_some(*l)
        })
        .take(15)
        .collect()
}
