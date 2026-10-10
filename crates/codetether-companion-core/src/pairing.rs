use crate::{Error, Registry, auth::TokenHash, input, secrets};
use codetether_companion_protocol::PairReceipt;

impl Registry {
    /// Consume a live pairing code and return a scoped device capability once.
    ///
    /// # Arguments
    /// * `code` - User-entered code; whitespace/hyphens are ignored.
    /// * `now` - Trusted Unix milliseconds.
    /// # Returns
    /// A 256-bit random URL-safe bearer token and the existing session deadline.
    /// Pairing does not authorize capture on the Windows machine.
    /// # Errors
    /// Returns pairing, attempt-limit, entropy, or internal clock errors.
    /// # Examples
    /// See [`Self::create`] for creating the owner receipt first.
    pub fn pair(&mut self, code: &str, now: i64) -> Result<PairReceipt, Error> {
        let code = input::normalize_code(code)?;
        if now.saturating_sub(self.window) >= 60_000 {
            self.attempts = 0;
            self.window = now;
        }
        self.attempts = self.attempts.saturating_add(1);
        if self.attempts > 30 {
            return Err(Error::Attempts);
        }
        let (id, session) = self
            .sessions
            .iter_mut()
            .find(|(_, session)| session.code == code && session.device.is_none())
            .ok_or(Error::Pairing)?;
        if session.pair_expires <= now || session.expires <= now || session.stopped {
            return Err(Error::Pairing);
        }
        let device_token = secrets::token()?;
        let expires_at = secrets::timestamp(session.expires)?;
        let interval_seconds = session.input.as_ref().ok_or(Error::Ended)?.interval_seconds;
        session.device = Some(TokenHash::new(&device_token));
        session.code.clear();
        Ok(PairReceipt {
            id: id.clone(),
            device_token,
            interval_seconds,
            expires_at,
        })
    }
}
