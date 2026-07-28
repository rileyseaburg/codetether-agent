//! Navigation lifecycle metadata for agent-controlled pages.

/// Deterministic page load milestones.
///
/// The variants are ordered by lifecycle progression so a later state satisfies
/// waits for earlier states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageLoadState {
    /// A new document was committed to the page.
    Commit,
    /// The committed document has a parsed DOM tree.
    DomContentLoaded,
    /// The deterministic in-memory load completed.
    Load,
    /// Script/network work known to the deterministic runtime has drained.
    NetworkIdle,
}

impl PageLoadState {
    /// Return true when this state is at or after `expected`.
    pub fn reached(self, expected: Self) -> bool {
        self >= expected
    }
}

/// Metadata about the current page navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageNavigation {
    /// Monotonic page-local navigation identifier.
    pub id: u64,
    /// Current page URL.
    pub url: String,
    /// Operation that produced this navigation record.
    pub action: String,
    /// Latest deterministic load state.
    pub state: PageLoadState,
}

impl PageNavigation {
    pub(crate) fn new(
        id: u64,
        url: impl Into<String>,
        action: impl Into<String>,
        state: PageLoadState,
    ) -> Self {
        Self {
            id,
            url: url.into(),
            action: action.into(),
            state,
        }
    }

    pub(crate) fn next(
        &self,
        url: impl Into<String>,
        action: impl Into<String>,
        state: PageLoadState,
    ) -> Self {
        Self::new(self.id + 1, url, action, state)
    }

    pub(crate) fn advance(&mut self, state: PageLoadState) {
        if state > self.state {
            self.state = state;
        }
    }
}
