/// Native attachment storage and text-placeholder expansion.
/// # Implementors
/// UI adapters retain image and paste-sidecar types.
pub trait Attachments {
    /// One pending image attachment.
    type Image;
    /// Pending text-paste sidecars.
    type TextPastes;
    /// Remove and return pending images.
    fn take_images(&mut self) -> Vec<Self::Image>;
    /// Remove and return pending text sidecars.
    fn take_text(&mut self) -> Self::TextPastes;
    /// Expand the prompt using the captured sidecars for the agent.
    fn expand(&self, prompt: &str, text: &Self::TextPastes) -> String;
}
