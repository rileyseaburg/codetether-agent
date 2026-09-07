//! Non-secret renewal failure categories; raw response bodies are never logged.
use vaultrs::error::ClientError;

#[derive(Debug, Clone, Copy)]
pub(super) enum Failure {
    Timeout,
    Http(u16),
    Transport,
}

#[cfg(test)]
mod tests {
    #[test]
    fn diagnostics_never_contain_vault_error_bodies() {
        let error = super::classify(vaultrs::error::ClientError::APIError {
            code: 403,
            errors: vec!["sensitive-fixture-value".into()],
        });
        assert!(error.terminal());
        assert!(!format!("{error:?}").contains("sensitive-fixture-value"));
        assert!(!super::Failure::Http(503).terminal());
        assert!(!super::Failure::Timeout.terminal());
    }
}

impl Failure {
    pub(super) fn terminal(self) -> bool {
        matches!(self, Self::Http(400 | 401 | 403 | 404))
    }
}

pub(super) fn classify(error: ClientError) -> Failure {
    match error {
        ClientError::APIError { code, .. } => Failure::Http(code),
        _ => Failure::Transport,
    }
}
