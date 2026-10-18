//! Frontend entry points for held-answer delivery and input interception.
/// Declare the held-answer delivery wrapper in a frontend entry module.
/// # Examples
/// See the frontend answer module for concrete state and registry types.
#[macro_export]
macro_rules! declare_frontend_answer_delivery {
    ($app:path, $slot:path, $registry:path $(,)?) => {
        use ::std::sync::Arc;
        use $app as App;
        use $registry as ProviderRegistry;
        use $slot as SessionSlot;
        pub(crate) async fn deliver(
            app: &mut App,
            slot: &SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
        ) -> bool {
            $crate::deliver(&mut super::delivery_host::Native {
                app,
                slot,
                registry,
            })
            .await
        }
    };
}
/// Declare the question-interception wrapper in a frontend entry module.
/// # Examples
/// See the frontend answer-submit module for concrete state and runtime types.
#[macro_export]
macro_rules! declare_frontend_answer_submit {
    ($app:path, $slot:path, $runtime:path $(,)?) => {
        use $app as App;
        use $runtime as TuiSessionHandle;
        use $slot as SessionSlot;
        pub(crate) async fn intercept(
            app: &mut App,
            slot: &SessionSlot,
            runtime: &TuiSessionHandle,
        ) -> bool {
            $crate::intercept(&mut super::input_host::Native { app, slot, runtime }).await
        }
    };
}
