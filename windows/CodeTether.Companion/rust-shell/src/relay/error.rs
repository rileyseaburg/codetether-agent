/// Redacted relay failure classification; never contains credentials or frame bytes.
pub(crate) enum Error {
    Cancelled,
    Conflict,
    InvalidInput,
    InvalidResponse,
    Rejected(u16),
    Revoked,
    Unavailable,
}

impl Error {
    pub(super) fn from_status(status: u16) -> Self {
        match status {
            401 | 403 | 404 | 410 => Self::Revoked,
            409 => Self::Conflict,
            200..=299 | 300..=399 => Self::InvalidResponse,
            _ => Self::Rejected(status),
        }
    }
    pub(super) fn from_ureq(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Status(401 | 403 | 404 | 410, _) => Self::Revoked,
            ureq::Error::Status(409, _) => Self::Conflict,
            ureq::Error::Status(status, _) => Self::Rejected(status),
            ureq::Error::Transport(_) => Self::Unavailable,
        }
    }

    pub(super) fn pair_message(self) -> String {
        match self {
            Self::Unavailable => "Cannot reach the relay. Check the network and try again.".into(),
            Self::InvalidResponse => "The relay returned an unexpected pairing response.".into(),
            Self::Revoked | Self::Conflict | Self::InvalidInput | Self::Cancelled => {
                "Pairing refused. Request a new code from your iPhone.".into()
            }
            Self::Rejected(status) => format!("Pairing refused by the relay (HTTP {status})."),
        }
    }

    /// Whether the device capability must be forgotten immediately.
    pub(crate) fn revoked(&self) -> bool {
        matches!(self, Self::Revoked)
    }
}
