//! Module script reference discovery.

use crate::browser::Document;

use super::{script_kind, walk};

pub(crate) fn collect(document: &Document) -> Vec<String> {
    let mut out = Vec::new();
    walk::elements(document, |element| {
        if script_kind::external(element) && script_kind::module(element) {
            if let Some(src) = element.attrs.get("src") {
                out.push(src.trim().into());
            }
        }
    });
    out
}
