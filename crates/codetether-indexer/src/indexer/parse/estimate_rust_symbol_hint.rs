//! estimate rust symbol hint for codebase indexing.

use super::*;

pub(in crate::indexer) fn estimate_rust_symbol_hint(line: &str) -> bool {
    let normalized = strip_prefixes(
        line,
        &[
            "pub(crate) ",
            "pub(super) ",
            "pub(self) ",
            "pub ",
            "async ",
            "unsafe ",
        ],
    );

    normalized.starts_with("fn ")
        || normalized.starts_with("struct ")
        || normalized.starts_with("enum ")
        || normalized.starts_with("trait ")
        || normalized.starts_with("impl ")
        || normalized.starts_with("mod ")
        || normalized.starts_with("type ")
        || normalized.starts_with("const ")
        || normalized.starts_with("static ")
}
