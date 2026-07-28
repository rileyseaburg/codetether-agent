#[derive(Clone)]
pub(super) struct Parts {
    pub(super) protocol: String,
    pub(super) hostname: String,
    pub(super) pathname: String,
    pub(super) search: String,
    pub(super) hash: String,
}

impl Parts {
    pub(super) fn any() -> Self {
        Self {
            protocol: "*".into(),
            hostname: "*".into(),
            pathname: "*".into(),
            search: "*".into(),
            hash: "*".into(),
        }
    }
}

#[derive(Clone)]
pub(super) struct Pattern {
    pub(super) parts: Parts,
}
