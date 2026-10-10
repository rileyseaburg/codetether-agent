use crate::{Error, auth::TokenHash};

/// Hashed owner credential, distinct from every session's device capability.
///
/// ```
/// use codetether_companion_core::OwnerCredential;
/// let owner = OwnerCredential::new("test-owner-credential-at-least-32-chars")?;
/// assert!(owner.authorize(Some("Bearer test-owner-credential-at-least-32-chars")).is_ok());
/// # Ok::<(), codetether_companion_core::Error>(())
/// ```
pub struct OwnerCredential(TokenHash);
impl OwnerCredential {
    /// Hash a configured owner token; raw credentials are not retained.
    ///
    /// # Arguments
    /// * `token` - Deployment-supplied credential, at least 32 UTF-16 code units.
    /// # Returns
    /// A verifier for owner-only operations.
    /// # Errors
    /// Returns [`Error::Configuration`] for short credentials.
    pub fn new(token: &str) -> Result<Self, Error> {
        if token.encode_utf16().count() < 32 {
            return Err(Error::Configuration);
        }
        Ok(Self(TokenHash::new(token)))
    }
    /// Verify a strict, case-sensitive `Bearer` header for an owner operation.
    ///
    /// # Errors
    /// Returns authentication-required or authentication-rejected failures.
    pub fn authorize(&self, authorization: Option<&str>) -> Result<(), Error> {
        self.0.verify(authorization)
    }
}
/// Reject cross-origin browser requests while allowing native clients.
///
/// # Arguments
/// * `origin` - Optional request Origin header; empty is treated as absent.
/// * `allowed` - Fixed deployment origin, not a value supplied by the requester.
/// # Returns
/// Success only for absent/empty Origin or an exact match.
/// # Errors
/// Returns [`Error::Origin`] for a nonempty mismatched origin.
/// # Examples
/// ```
/// assert!(codetether_companion_core::require_origin(None, "https://example.test").is_ok());
/// ```
pub fn require_origin(origin: Option<&str>, allowed: &str) -> Result<(), Error> {
    if origin.is_some_and(|value| !value.is_empty() && value != allowed) {
        Err(Error::Origin)
    } else {
        Ok(())
    }
}
