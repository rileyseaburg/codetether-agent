//! Frontend state and review-context bindings for incoming approvals.

/// Declare approval presentation using existing diagnostics and review services.
///
/// The context type supplies workspace, session_id and session_model fields.
/// Diagnostics, reviewer dispatch and trust guidance remain application choices;
/// the shared request workflow retains queue and presentation ordering.
/// # Examples
/// ```text
/// codetether_approval_view::declare_request_view!(
///     App, start_analysis, guidance, start_review, ReviewContext,
/// );
/// ```
#[macro_export]
macro_rules! declare_request_view {
    ($app:ty, $diagnostics:path, $guidance:path, $dispatch:path, $context:path $(,)?) => {
        mod host {
            use $crate::__private::ApprovalSnapshot;
            pub(super) struct Native<'a>(pub &'a mut $app);
            impl $crate::request::View for Native<'_> {
                fn reset_preview_scroll(&mut self) {
                    self.0.state.approval_preview_scroll = 0;
                }
                fn set_waiting(&mut self, waiting: bool) {
                    self.0.state.approval_waiting = waiting;
                }
                fn start_analysis(&mut self, item: &ApprovalSnapshot) {
                    $diagnostics(
                        &mut self.0.state.editor_lsp.manager,
                        &self.0.state.cwd_display,
                        item,
                    );
                }
                fn start_review(&mut self, item: &ApprovalSnapshot) {
                    super::review::start(self.0, item);
                }
                fn approval_guidance(&self) -> String {
                    $guidance()
                }
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
            }
        }

        mod review {
            use $context as ReviewContext;
            pub(super) fn start(app: &mut $app, item: &$crate::__private::ApprovalSnapshot) {
                $dispatch(item, || ReviewContext {
                    workspace: ::std::path::PathBuf::from(&app.state.cwd_display),
                    session_id: app.state.session_id.clone(),
                    session_model: app.state.last_completion_model.clone(),
                });
            }
        }

        pub(super) fn request(app: &mut $app, request: $crate::__private::LiveApprovalRequest) {
            $crate::request::request(&mut host::Native(app), request);
        }
    };
}
