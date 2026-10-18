//! Frontend journal binding for human-answer workflows.
/// Implementation detail of [crate::declare_frontend_answer_hosts].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_answer_journal {
    ($state:path, $action:path; $read:path, $held:path, $begin:path, $record:path) => {
        mod review_host {
            use $action as AnswerReviewAction;
            use $state as TaskState;
            pub(super) struct Native;
            impl $crate::Journal for Native {
                fn read(id: &str) -> anyhow::Result<TaskState> {
                    $read(id)
                }
                fn held(id: &str) -> bool {
                    $held(id)
                }
                async fn begin(id: &str, question: &str) -> anyhow::Result<bool> {
                    $begin(id, question).await
                }
                async fn record(
                    id: &str,
                    goal: &str,
                    review: &str,
                    action: AnswerReviewAction,
                ) -> anyhow::Result<()> {
                    $record(id, goal, review, action).await
                }
            }
        }
    };
}
