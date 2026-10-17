//! key from env for verified plugin registration.
use super::*;
impl SigningKey {
    /// Load the HMAC key from `CODETETHER_PLUGIN_SIGNING_KEY` or generate one.
    pub fn from_env() -> Self {
        let key = match std::env::var("CODETETHER_PLUGIN_SIGNING_KEY") {
            Ok(hex) if hex.len() >= 32 => {
                tracing::info!(target: "codetether_agent::tool::sandbox","Plugin signing key loaded from environment");
                hex.into_bytes()
            }
            _ => {
                let mut rng = rand::rng();
                let key: Vec<u8> = (0..32)
                    .map(|_| rand::RngExt::random::<u8>(&mut rng))
                    .collect();
                tracing::warn!(target: "codetether_agent::tool::sandbox",
                    "No CODETETHER_PLUGIN_SIGNING_KEY set — generated ephemeral key. \
                     Plugin signatures will not persist across restarts."
                );
                key
            }
        };
        Self { key: Arc::new(key) }
    }
}
