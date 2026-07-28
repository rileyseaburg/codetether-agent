//! Stable diagnostics for locator kinds.

use std::fmt;

use crate::browser_agent::locator::LocatorKind;

impl fmt::Debug for LocatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Css(value) => write!(f, "css({value:?})"),
            Self::Text(value) => write!(f, "text_contains({value:?})"),
            Self::TextExact(value) => write!(f, "text_exact({value:?})"),
            Self::Role(value) => write!(f, "get_by_role({value:?})"),
            Self::RoleName { role, name } => write!(f, "get_by_role({role:?}, name={name:?})"),
            Self::TestId(value) => write!(f, "get_by_test_id({value:?})"),
            Self::Label(value) => write!(f, "get_by_label({value:?})"),
            Self::Placeholder(value) => write!(f, "get_by_placeholder({value:?})"),
            Self::AltText(value) => write!(f, "get_by_alt_text({value:?})"),
            Self::Title(value) => write!(f, "get_by_title({value:?})"),
        }
    }
}
