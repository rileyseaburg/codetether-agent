//! Page region detection.

mod types;

use super::ElementSummary;
pub use types::{PageRegion, RegionInfo};

fn has(s: &str, n: &str) -> bool {
    s.to_ascii_lowercase().contains(n)
}

fn region_for(e: &ElementSummary) -> Option<(PageRegion, f64)> {
    let tag = e.tag.to_ascii_lowercase();
    let role = e.role.clone().unwrap_or_default().to_ascii_lowercase();
    let id = e.id.clone().unwrap_or_default();
    let all = format!("{} {} {} {}", tag, role, id, e.classes.join(" "));
    if tag == "header" || role == "banner" {
        return Some((PageRegion::Header, 0.95));
    }
    if tag == "nav" || role == "navigation" {
        return Some((PageRegion::Navigation, 0.95));
    }
    if tag == "main" || role == "main" {
        return Some((PageRegion::Main, 0.95));
    }
    if tag == "footer" || role == "contentinfo" {
        return Some((PageRegion::Footer, 0.95));
    }
    if role == "dialog" {
        return Some((PageRegion::Dialog, 0.95));
    }
    if has(&all, "modal") {
        return Some((PageRegion::Modal, 0.80));
    }
    if has(&all, "sidebar") || has(&all, "aside") {
        return Some((PageRegion::Sidebar, 0.75));
    }
    if tag == "aside" || role == "complementary" {
        return Some((PageRegion::Complementary, 0.85));
    }
    None
}

pub fn detect_regions(elements: &[ElementSummary]) -> Vec<RegionInfo> {
    elements
        .iter()
        .filter_map(|e| RegionInfo::from_match(e.selector.clone(), region_for(e)?))
        .collect()
}
