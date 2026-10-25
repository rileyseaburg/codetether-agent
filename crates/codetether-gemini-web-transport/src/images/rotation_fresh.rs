pub(super) fn fresh(updated: u64, instant: u64) -> bool {
    updated > 0 && updated <= instant && instant - updated < 60
}

pub(super) fn usable(updated: u64, cookies: &[super::rotation::Rotation], instant: u64) -> bool {
    fresh(updated, instant)
        && cookies
            .iter()
            .any(|c| c.name == "__Secure-1PSIDTS" && c.expires > instant && !c.value.is_empty())
}
