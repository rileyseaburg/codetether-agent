//! Validated transcript inheritance selector for spawned agents.

use anyhow::{Result, bail};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tool::agent) enum ForkTurns {
    None,
    All,
    Count(usize),
}

impl ForkTurns {
    pub(in crate::tool::agent) fn parse(value: Option<&str>) -> Result<Self> {
        match value.unwrap_or("all").trim() {
            "none" => Ok(Self::None),
            "all" => Ok(Self::All),
            value => match value.parse::<usize>() {
                Ok(count) if count > 0 => Ok(Self::Count(count)),
                _ => bail!("fork_turns must be none, all, or a positive integer"),
            },
        }
    }
}

#[cfg(test)]
#[path = "fork_turns_tests.rs"]
mod tests;
