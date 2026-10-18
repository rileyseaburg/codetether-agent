//! Lazily loaded syntax and theme assets for approval diff highlighting.

use std::sync::OnceLock;
use syntect::{highlighting::Theme, parsing::SyntaxSet};

pub(super) fn syntaxes() -> &'static SyntaxSet {
    static SET: OnceLock<SyntaxSet> = OnceLock::new();
    SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

pub(super) fn syntax(path: &str) -> Option<&'static syntect::parsing::SyntaxReference> {
    let set = syntaxes();
    if let Ok(Some(syntax)) = set.find_syntax_for_file(path) {
        return Some(syntax);
    }
    match std::path::Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
    {
        Some("ts" | "tsx") => set.find_syntax_by_extension("js"),
        _ => None,
    }
}

pub(super) fn theme() -> &'static Theme {
    static THEME: OnceLock<Theme> = OnceLock::new();
    THEME.get_or_init(|| {
        syntect::highlighting::ThemeSet::load_defaults().themes["base16-ocean.dark"].clone()
    })
}
