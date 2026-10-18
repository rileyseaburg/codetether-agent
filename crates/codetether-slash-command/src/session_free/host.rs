//! UI-only operations available without a checked-out session.
/// Application operations allowed by session-free slash dispatch.
pub trait Host {
    /// Attach a file using the cleaned command argument.
    fn attach_file(&mut self, path: &str);
    /// Attach an image using the cleaned command argument.
    fn attach_image(&mut self, path: &str);
    /// Open the settings view.
    fn open_settings(&mut self);
    /// Open the model picker without session mutation.
    fn open_model(&mut self);
    /// Show help and reset its scroll position.
    fn open_help(&mut self);
}
