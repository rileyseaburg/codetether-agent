//! Frontend input operations for steering.
/// Implementation detail of [crate::declare_frontend_steering].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_steering_input {
    ($image:path, $steering:path, $expand:path, $queue:path) => {
        mod input {
            use super::Native;
            use $image as ImageAttachment;
            use $steering as SteeringInput;
            impl $crate::steering::Input for Native<'_> {
                fn images(&self) -> Vec<ImageAttachment> {
                    self.app.state.attachments.images.clone()
                }
                fn expand(&self, prompt: &str) -> String {
                    $expand(prompt, &self.app.state.attachments.text)
                }
                fn steer(&self, expanded: String, images: Vec<ImageAttachment>) -> bool {
                    self.runtime
                        .steer_current(SteeringInput::new(expanded, images))
                }
                fn queue(&mut self, prompt: String) {
                    $queue(self.app, prompt);
                }
            }
        }
    };
}
