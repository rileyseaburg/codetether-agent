use super::*;

pub(crate) fn set_document_cookie(raw: &str) {
    let Some((name, rest)) = raw.split_once('=') else {
        return;
    };
    let name = name.trim();
    if name.is_empty() {
        return;
    }
    state::push_mutation(raw.into());
    super::super::network_cookie_host::apply_document_cookie(raw);
    update::visible(
        name,
        rest.split(';').next().unwrap_or_default().trim(),
        delete::deletes(raw),
    );
}

pub(super) fn set_pair(name: &str, value: &str) {
    let name = name.trim();
    if name.is_empty() {
        return;
    }
    state::push_mutation(format!("{name}={value}"));
    super::super::network_cookie_host::apply_document_cookie(&format!("{name}={value}"));
    update::visible(name, value, false);
}

pub(super) fn delete_pair(name: &str) {
    let name = name.trim();
    if name.is_empty() {
        return;
    }
    state::push_mutation(format!("{name}=; Max-Age=0"));
    super::super::network_cookie_host::apply_document_cookie(&format!("{name}=; Max-Age=0"));
    update::visible(name, "", true);
}
