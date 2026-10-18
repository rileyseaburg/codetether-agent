//! Native trust, reviewer and session configuration effects.
use codetether_config::Config;
/// Operations surrounding application of startup policy.
/// # Implementors
/// Frontends retain their registry type and active session ownership.
pub trait Host {
    /// Native provider registry used by session and reviewer setup.
    type Registry;
    /// Publish the effective trust-policy snapshot.
    fn set_trust(&mut self, config: &Config);
    /// Install the reviewer using the loaded registry.
    fn install_review(&mut self, config: &Config, registry: &Self::Registry);
    /// Apply loaded configuration to the active session.
    fn apply_config(&mut self, config: &Config, registry: Option<&Self::Registry>);
    /// Enable automatic edits after normal configuration has been applied.
    fn enable_auto_apply(&mut self);
}
