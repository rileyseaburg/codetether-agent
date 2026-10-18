use codetether_image::ImageAttachment;
/// Native pending-input access and active-turn transport.
/// # Implementors
/// UI adapters retain runtime steering and follow-up queue ownership.
pub trait Input {
    /// Clone pending images without removing originals before acceptance.
    fn images(&self) -> Vec<ImageAttachment>;
    /// Expand text-paste placeholders for the active agent.
    fn expand(&self, prompt: &str) -> String;
    /// Attempt to send the expanded text and image copies to the active run.
    fn steer(&self, expanded: String, images: Vec<ImageAttachment>) -> bool;
    /// Queue raw text for a later turn after steering fails.
    fn queue(&mut self, prompt: String);
}
