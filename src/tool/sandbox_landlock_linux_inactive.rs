use super::Prepared;

pub(super) fn prepared(reason: &'static str) -> Prepared {
    Prepared {
        rules: None,
        fallback: Some(format!("landlock_inactive:{reason}")),
    }
}
