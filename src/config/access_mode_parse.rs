use super::AccessMode;
use std::str::FromStr;

impl FromStr for AccessMode {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ask" => Ok(Self::Ask),
            "approve" => Ok(Self::Approve),
            "full" => Ok(Self::Full),
            _ => anyhow::bail!("access_mode must be ask, approve, or full"),
        }
    }
}
