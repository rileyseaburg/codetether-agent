#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RemoveOutcome {
    Removed,
    RefusedDirty,
    Failed,
}

impl RemoveOutcome {
    pub(crate) fn removed(self) -> bool {
        matches!(self, Self::Removed)
    }
}
