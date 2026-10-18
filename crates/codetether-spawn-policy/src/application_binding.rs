//! Application store and provider bindings for spawn admission.

/// Declare owner-scoped identity reservation using existing live/durable stores.
///
/// # Examples
/// ```text
/// codetether_spawn_policy::declare_identity_reservation!(pub(super), store, persistence);
/// ```
#[macro_export]
macro_rules! declare_identity_reservation {
    ($visibility:vis, $store:path, $persistence:path $(,)?) => {
        $visibility use $crate::Reservation;
        mod host {
            use $store as store;
            use $persistence as persistence;
            pub struct ApplicationHost;
            #[$crate::__private::async_trait]
            impl $crate::IdentityHost for ApplicationHost {
                async fn persisted_exists(owner: Option<&str>, name: &str) -> $crate::__private::Result<bool> {
                    persistence::exists(owner, name).await
                }
                fn contains_name(name: &str, owner: Option<&str>) -> bool {
                    store::contains_name_for_owner(name, owner)
                }
            }
        }
        $visibility async fn reserve(name: &str, owner: Option<&str>) -> ::std::result::Result<Reservation, $crate::__private::ToolResult> {
            $crate::reserve::<host::ApplicationHost>(name, owner).await
        }
    };
}

/// Declare depth/model admission using the existing lineage and provider source.
///
/// The request type supplies the child name, parent session and model. The
/// provider callback returns its future without eagerly loading providers, so
/// the shared admission sequence still checks depth first.
///
/// # Examples
/// ```text
/// codetether_spawn_policy::declare_spawn_validation!(
///     pub(super), SpawnRequest<'_>, lineage_for_session, get_registry,
/// );
/// ```
#[macro_export]
macro_rules! declare_spawn_validation {
    ($visibility:vis, $request:ty, $lineage:path, $registry:path $(,)?) => {
        /// Validate depth and return an optional advisory model-cost warning.
        $visibility async fn validate_spawn_request(
            request: &$request,
        ) -> ::std::result::Result<Option<String>, $crate::__private::ToolResult> {
            let (_, depth) = $lineage(request.parent_session_id);
            $crate::admission::validate(
                request.name,
                depth,
                request.model,
                $registry(),
            )
            .await
        }
    };
}
