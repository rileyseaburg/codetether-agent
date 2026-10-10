use crate::Error;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

pub(crate) struct TokenHash([u8; 32]);
impl TokenHash {
    pub(crate) fn new(token: &str) -> Self {
        Self(Sha256::digest(token.as_bytes()).into())
    }
    pub(crate) fn verify(&self, authorization: Option<&str>) -> Result<(), Error> {
        let token = authorization
            .and_then(|header| header.strip_prefix("Bearer "))
            .filter(|token| {
                !token.is_empty()
                    && token
                        .bytes()
                        .all(|c| c.is_ascii_alphanumeric() || b"._~-".contains(&c))
            })
            .ok_or(Error::AuthenticationRequired)?;
        if bool::from(self.0.ct_eq(&Self::new(token).0)) {
            Ok(())
        } else {
            Err(Error::AuthenticationRejected)
        }
    }
}
