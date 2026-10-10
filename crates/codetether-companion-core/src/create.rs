use crate::{Error, Registry, Session, input, secrets};
use codetether_companion_protocol::{SessionInput, SessionReceipt};

impl Registry {
    /// Validate input, sweep ended sessions, and issue a one-use pairing code.
    ///
    /// # Arguments
    /// * `input` - Owner-selected model, prompt, and interval.
    /// * `now` - Trusted Unix milliseconds.
    /// # Returns
    /// Owner receipt with five-minute pairing and one-hour session deadlines.
    /// # Errors
    /// Returns input, capacity, entropy, or internal clock errors.
    /// # Examples
    /// See the crate-level example.
    pub fn create(&mut self, input: SessionInput, now: i64) -> Result<SessionReceipt, Error> {
        let input = input::validate(input)?;
        let pair_expires = now.checked_add(300_000).ok_or(Error::Configuration)?;
        let expires = now.checked_add(3_600_000).ok_or(Error::Configuration)?;
        let pair_expires_at = secrets::timestamp(pair_expires)?;
        let expires_at = secrets::timestamp(expires)?;
        self.sweep(now);
        if self.sessions.len() >= 4 {
            return Err(Error::Capacity);
        }
        let id = secrets::id()?;
        let mut code = secrets::code()?;
        while self.sessions.values().any(|session| session.code == code) {
            code = secrets::code()?;
        }
        let receipt = SessionReceipt {
            id: id.clone(),
            code: code.clone(),
            pair_expires_at,
            expires_at,
            interval_seconds: input.interval_seconds,
        };
        self.sessions.insert(
            id,
            Session {
                input: Some(input),
                code,
                pair_expires,
                expires,
                device: None,
                stopped: false,
            },
        );
        Ok(receipt)
    }
}
