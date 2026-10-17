//! all skills are unique and non empty derivation policy.
use super::*;

#[test]
pub(super) fn all_skills_are_unique_and_non_empty() {
    let mut copy: Vec<&str> = ALL.to_vec();
    copy.sort_unstable();
    copy.dedup();
    assert_eq!(copy.len(), ALL.len(), "skill names must be unique");
    for name in ALL {
        assert!(!name.is_empty());
        assert_eq!(
            name.to_ascii_lowercase(),
            *name,
            "skills must be snake_case"
        );
    }
}
