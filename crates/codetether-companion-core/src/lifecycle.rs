use crate::{Error, Registry, Session};

impl Registry {
    /// Look up a live session, revoking it when its deadline is reached.
    ///
    /// # Arguments
    /// * `id` - Server-generated session UUID.
    /// * `now` - Trusted Unix milliseconds.
    /// # Returns
    /// Borrowed session state; no owner authorization is performed here.
    /// # Errors
    /// Returns [`Error::NotFound`] or [`Error::Ended`].
    /// # Examples
    /// See [`Session`] for reading state after creating a session.
    pub fn session(&mut self, id: &str, now: i64) -> Result<&Session, Error> {
        let session = self.sessions.get_mut(id).ok_or(Error::NotFound)?;
        if session.expires <= now || session.stopped {
            session.stop();
            return Err(Error::Ended);
        }
        Ok(session)
    }
    /// Check a device bearer only against this live session's capability.
    ///
    /// The caller must apply origin policy separately. This never grants owner
    /// access or local capture consent.
    /// # Errors
    /// Returns session lookup or authentication failures.
    pub fn authorize_device(
        &mut self,
        id: &str,
        authorization: Option<&str>,
        now: i64,
    ) -> Result<(), Error> {
        self.session(id, now)?
            .device
            .as_ref()
            .ok_or(Error::AuthenticationRejected)?
            .verify(authorization)
    }
    /// Revoke a session and discard its capability and owner instructions.
    ///
    /// The caller must first authorize the owner; this is not an HTTP handler.
    /// # Errors
    /// Returns [`Error::NotFound`] or [`Error::Ended`] for a non-live session.
    pub fn stop(&mut self, id: &str, now: i64) -> Result<(), Error> {
        self.session(id, now)?;
        self.sessions.get_mut(id).ok_or(Error::NotFound)?.stop();
        Ok(())
    }
    /// Discard ended entries, revoking their device capabilities.
    pub fn sweep(&mut self, now: i64) {
        self.sessions
            .retain(|_, session| !session.stopped && session.expires > now);
    }
}
