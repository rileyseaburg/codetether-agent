//! Source-mapped stack frame construction.

use crate::browser_agent::page::BrowserPage;

use super::mapped_types::{GeneratedSourceLocation, SourceMappedStackFrame};

pub fn frames(
    page: &BrowserPage,
    generated: &GeneratedSourceLocation,
) -> Vec<SourceMappedStackFrame> {
    let Some(source) = super::mapped_source::script(page, &generated.script_url) else {
        return vec![frame(page, None, generated.clone())];
    };
    super::mapped_stack_scan::frames(source, generated)
        .into_iter()
        .map(|(name, generated)| frame(page, name, generated))
        .collect()
}

fn frame(
    page: &BrowserPage,
    function_name: Option<String>,
    generated: GeneratedSourceLocation,
) -> SourceMappedStackFrame {
    let original = super::source_maps::map_location(page, &generated);
    SourceMappedStackFrame {
        function_name,
        generated,
        original,
    }
}
