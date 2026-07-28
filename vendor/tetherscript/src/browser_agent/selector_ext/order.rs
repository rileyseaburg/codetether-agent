//! Order-dependent selector extension filters.

use crate::browser_agent::locator::{Locator, LocatorKind};
use crate::browser_agent::query::DomMatch;

use super::parse;

pub(crate) fn apply(locator: &Locator, matches: Vec<DomMatch>) -> Vec<DomMatch> {
    let LocatorKind::Css(selector) = &locator.kind else {
        return matches;
    };
    let plan = parse::parse(selector);
    if plan.invalid() {
        return Vec::new();
    }
    let Some(index) = plan.nth() else {
        return matches;
    };
    matches.into_iter().nth(index).into_iter().collect()
}
