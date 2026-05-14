//! Title and document text helpers.

use tetherscript::browser::{query_selector, text_content};

/// Return the current page title.
pub(in crate::browser::session::native) fn title(page: &super::super::NativePage) -> String {
    query_selector(&page.session.document, "title")
        .first()
        .map(text_content)
        .unwrap_or_default()
}

/// Return all visible text from the document tree.
pub(super) fn document_text(page: &super::super::NativePage) -> String {
    page.session
        .document
        .children
        .iter()
        .map(text_content)
        .collect::<Vec<_>>()
        .join(" ")
}
