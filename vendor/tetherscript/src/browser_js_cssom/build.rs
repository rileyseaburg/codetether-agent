use crate::browser::Document;

use super::collect;
use super::model::Sheet;
use super::parse_rule;

pub(super) fn sheets(document: &Document, css: &str) -> Vec<Sheet> {
    let mut sources = collect::style_sources(document);
    append_extra_css(&mut sources, css);
    sources
        .into_iter()
        .map(|source| Sheet {
            rules: parse_rule::rules(&source),
        })
        .collect()
}

fn append_extra_css(sources: &mut Vec<String>, css: &str) {
    if css.trim().is_empty() {
        return;
    }
    let embedded = sources.join("\n");
    if embedded.trim().is_empty() {
        sources.push(css.to_string());
    } else if css == embedded {
    } else if let Some(extra) = css.strip_prefix(&format!("{embedded}\n")) {
        push_nonempty(sources, extra);
    } else if !sources.iter().any(|source| source == css) {
        sources.push(css.to_string());
    }
}

fn push_nonempty(sources: &mut Vec<String>, css: &str) {
    if !css.trim().is_empty() {
        sources.push(css.to_string());
    }
}
