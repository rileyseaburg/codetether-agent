//! Best-effort generated source location extraction for page errors.

use crate::browser_agent::events::PageErrorEvent;
use crate::browser_agent::page::resources::{ResourceKind, ResourcePayload};
use crate::browser_agent::page::BrowserPage;

use super::mapped_types::GeneratedSourceLocation;

pub fn location(page: &BrowserPage, error: &PageErrorEvent) -> Option<GeneratedSourceLocation> {
    let name = super::mapped_error_parse::reference_name(&error.message);
    let explicit = super::mapped_error_parse::explicit_position(&error.message);
    if let Some((url, source)) = script_source(page, name.as_deref()) {
        let (line, column) = explicit
            .or_else(|| name.and_then(|item| super::mapped_error_parse::find(&source, &item)))?;
        return Some(GeneratedSourceLocation {
            script_url: url,
            line,
            column,
        });
    }
    explicit.map(|(line, column)| GeneratedSourceLocation {
        script_url: "eval".into(),
        line,
        column,
    })
}

fn script_source(page: &BrowserPage, name: Option<&str>) -> Option<(String, String)> {
    page.resources.entries().iter().find_map(|resource| {
        let ResourcePayload::Text(source) = &resource.payload else {
            return None;
        };
        if resource.kind != ResourceKind::Script {
            return None;
        }
        if name.is_some_and(|item| !source.contains(item)) {
            return None;
        }
        Some((resource.url.clone(), source.clone()))
    })
}
