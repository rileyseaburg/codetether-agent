//! Public page history entry metadata.

use super::result::NavigationKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageHistoryEntry {
    pub index: usize,
    pub url: String,
    pub navigation_id: u64,
    pub kind: NavigationKind,
    pub current: bool,
}

impl PageHistoryEntry {
    pub(crate) fn new(
        index: usize,
        url: String,
        navigation_id: u64,
        kind: NavigationKind,
        current: bool,
    ) -> Self {
        Self {
            index,
            url,
            navigation_id,
            kind,
            current,
        }
    }
}
