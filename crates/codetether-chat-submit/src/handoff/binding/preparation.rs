//! Frontend preparation operations for handoff.
/// Implementation detail of [crate::declare_frontend_handoff].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_handoff_preparation {
    ($image:path, $request:path) => {
        mod preparation {
            use super::Native;
            use ::std::{path::PathBuf, sync::Arc};
            use $image as ImageAttachment;
            use $request as PromptRequest;
            impl $crate::handoff::Preparation for Native<'_> {
                type Image = ImageAttachment;
                fn original_directory(&self) -> Option<Option<PathBuf>> {
                    self.slot.borrow().map(|s| s.metadata.directory.clone())
                }
                fn use_worktree(&self) -> bool {
                    self.app.state.use_worktree
                }
                fn configure(&mut self, use_worktree: bool) {
                    if let Some(session) = self.slot.borrow_mut() {
                        session.metadata.auto_apply_edits = self.app.state.auto_apply_edits;
                        session.metadata.use_worktree = use_worktree;
                    }
                }
                fn take_session(&mut self) -> Option<Self::Session> {
                    self.slot.take_for_prompt()
                }
                fn request(
                    &self,
                    session: Self::Session,
                    prompt: String,
                    images: Vec<Self::Image>,
                    original_dir: Option<PathBuf>,
                    worktree_root: Option<PathBuf>,
                ) -> Self::Request {
                    PromptRequest::new(
                        session,
                        prompt,
                        images,
                        Arc::clone(self.registry),
                        original_dir,
                        worktree_root,
                    )
                }
            }
        }
    };
}
