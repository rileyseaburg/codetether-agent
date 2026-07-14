//! Filesystem capabilities assigned to a swarm participant.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Capability {
    ReadOnly,
    Verification,
    Mutating,
}

impl Capability {
    pub(crate) fn from_flags(read_only: bool, verification: bool) -> Self {
        match (read_only, verification) {
            (true, _) => Self::ReadOnly,
            (false, true) => Self::Verification,
            (false, false) => Self::Mutating,
        }
    }

    pub(super) fn allows_commit(self) -> bool {
        self == Self::Mutating
    }

    pub(crate) fn is_mutating(self) -> bool {
        self == Self::Mutating
    }
}
