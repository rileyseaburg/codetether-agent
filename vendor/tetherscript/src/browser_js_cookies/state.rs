use super::*;

pub(crate) fn reset() {
    COOKIE_JAR.with(|cookies| cookies.borrow_mut().clear());
    COOKIE_MUTATIONS.with(|mutations| mutations.borrow_mut().clear());
}

pub(crate) fn seed(cookies: Vec<(String, String)>) {
    COOKIE_JAR.with(|jar| *jar.borrow_mut() = cookies);
    COOKIE_MUTATIONS.with(|mutations| mutations.borrow_mut().clear());
}

pub(crate) fn visible_pairs() -> Vec<(String, String)> {
    COOKIE_JAR.with(|cookies| cookies.borrow().clone())
}

pub(crate) fn mutations() -> Vec<String> {
    COOKIE_MUTATIONS.with(|mutations| mutations.borrow().clone())
}

pub(crate) fn cookie_string() -> String {
    visible_pairs()
        .into_iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join("; ")
}

pub(super) fn push_mutation(raw: String) {
    COOKIE_MUTATIONS.with(|mutations| mutations.borrow_mut().push(raw));
}
