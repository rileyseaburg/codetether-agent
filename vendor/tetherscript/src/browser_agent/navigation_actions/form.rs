//! GET form navigation target resolution.

use crate::browser::Document;

pub(crate) fn submit_target(
    document: &Document,
    submitter_path: &[usize],
    current_url: &str,
) -> Option<String> {
    let submitter = super::dom::element_at_path(document, submitter_path)?;
    let (_, form) = super::dom::closest_form(document, submitter_path)?;
    target_for_form(form, Some(submitter), current_url)
}

pub(crate) fn form_target(
    document: &Document,
    form_path: &[usize],
    current_url: &str,
) -> Option<String> {
    let form = super::dom::element_at_path(document, form_path)?;
    if form.tag.eq_ignore_ascii_case("form") {
        target_for_form(form, None, current_url)
    } else {
        None
    }
}

fn target_for_form(
    form: &crate::browser::Element,
    submitter: Option<&crate::browser::Element>,
    current_url: &str,
) -> Option<String> {
    let method = form
        .attrs
        .get("method")
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| "get".into());
    if method != "get" {
        return None;
    }
    let action = form.attrs.get("action").map_or(current_url, String::as_str);
    let url = super::url::resolve(current_url, action);
    let entries = super::entries::collect(form, submitter);
    Some(super::query::with_query(&url, &entries))
}
