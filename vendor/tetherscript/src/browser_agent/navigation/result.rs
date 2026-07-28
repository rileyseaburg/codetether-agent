//! Navigation result and status values.

use crate::browser_agent::navigation_state::{PageLoadState, PageNavigation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationKind {
    Initial,
    DocumentReplacement,
    SameDocument,
    Reload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationStatus {
    Committed,
    NoEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationResult {
    pub navigation: PageNavigation,
    pub kind: NavigationKind,
    pub status: NavigationStatus,
}

impl NavigationResult {
    pub(crate) fn committed(navigation: PageNavigation, kind: NavigationKind) -> Self {
        Self {
            navigation,
            kind,
            status: NavigationStatus::Committed,
        }
    }

    pub(crate) fn no_entry(navigation: PageNavigation) -> Self {
        Self {
            navigation,
            kind: NavigationKind::Initial,
            status: NavigationStatus::NoEntry,
        }
    }

    pub fn wait_for_load_state(&self, state: PageLoadState) -> Result<PageNavigation, String> {
        if self.status != NavigationStatus::Committed {
            return Err("navigation did not commit a history entry".into());
        }
        if self.navigation.state.reached(state) {
            Ok(self.navigation.clone())
        } else {
            Err(format!(
                "navigation {} at {} is in {:?}, not {:?}",
                self.navigation.id, self.navigation.url, self.navigation.state, state
            ))
        }
    }
}
