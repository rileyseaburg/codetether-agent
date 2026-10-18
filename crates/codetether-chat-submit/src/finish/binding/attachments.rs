//! Frontend attachments for prompt finish.
/// Implementation detail of [crate::declare_frontend_prompt_finish].
#[doc(hidden)]
#[macro_export]
macro_rules! __prompt_finish_attachments {
    ($image:path, $paste:path, $expand:path) => {
        mod attachments {
            use super::Native;
            use $image as ImageAttachment;
            use $paste as PendingTextPaste;
            impl $crate::finish::Attachments for Native<'_> {
                type Image = ImageAttachment;
                type TextPastes = Vec<PendingTextPaste>;
                fn take_images(&mut self) -> Vec<Self::Image> {
                    self.app.state.attachments.take_images()
                }
                fn take_text(&mut self) -> Self::TextPastes {
                    self.app.state.attachments.take_text()
                }
                fn expand(&self, prompt: &str, text: &Self::TextPastes) -> String {
                    $expand(prompt, text)
                }
            }
        }
    };
}
