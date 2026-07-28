//! Source-map registration status helpers.

use crate::browser_agent::page::resources::{ResourceKind, ResourcePayload};
use crate::browser_agent::page::BrowserPage;

use super::super::types::SourceMapStatus;

pub fn statuses(page: &BrowserPage) -> Vec<SourceMapStatus> {
    page.resources
        .entries()
        .iter()
        .filter_map(|resource| match &resource.payload {
            ResourcePayload::Text(source) if resource.kind == ResourceKind::Script => {
                mapping_url(source).map(|url| status(page, &resource.url, &url))
            }
            _ => None,
        })
        .collect()
}

pub fn source_map_text<'a>(page: &'a BrowserPage, script_url: &str) -> Option<&'a str> {
    let source = page.resources.text(ResourceKind::Script, script_url)?;
    let reference = mapping_url(source)?;
    let url = resolve(script_url, &reference);
    page.resources
        .text(ResourceKind::SourceMap, &reference)
        .or_else(|| page.resources.text(ResourceKind::SourceMap, &url))
}

fn status(page: &BrowserPage, script_url: &str, reference: &str) -> SourceMapStatus {
    let url = resolve(script_url, reference);
    SourceMapStatus {
        script_url: script_url.into(),
        source_map_url: url.clone(),
        registered: has_map(page, reference) || has_map(page, &url),
    }
}

fn mapping_url(source: &str) -> Option<String> {
    source.lines().rev().find_map(|line| {
        line.split_once("sourceMappingURL=")
            .map(|(_, value)| value.trim().to_string())
    })
}

fn has_map(page: &BrowserPage, url: &str) -> bool {
    page.resources.has(ResourceKind::SourceMap, url)
}

fn resolve(script_url: &str, reference: &str) -> String {
    if reference.contains("://") || reference.starts_with('/') {
        return reference.into();
    }
    script_url
        .rsplit_once('/')
        .map(|(base, _)| format!("{base}/{reference}"))
        .unwrap_or_else(|| reference.into())
}
