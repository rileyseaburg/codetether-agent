//! Application bindings for the child-turn runtime traits.
mod finalize;
mod mailbox;
mod message;
mod prompt;
mod runtime;
mod startup;

/// Bind child-turn runtime traits to existing application types and services.
///
/// Full type paths and service callbacks preserve the embedding application's
/// session, event stream, registry, storage, steering and mailbox owners.
/// # Examples
/// ```text
/// codetether_child_turn::impl_child_turn!(
///     AppHost, Session, SessionEvent, ProviderRegistry;
///     store, bus, mailbox, clear_steering, open_steering, announce, finished;
///     get_registry, begin_first, begin_message, remote, prepare, ensure_bus,
/// );
/// ```
#[macro_export]
macro_rules! impl_child_turn {
    ($host:ty, $session:ty, $event:ty, $registry:ty;
     $store:path, $bus:path, $queue:path, $clear:path, $open:path, $announce:path, $finish:path;
     $get:path, $first:path, $message:path, $remote:path, $prepare:path, $ensure:path $(,)?) => {
        $crate::__impl_child_finalize!($host, $session, $store, $bus, $clear);
        $crate::__impl_child_mailbox!($host, $queue);
        $crate::__impl_child_message!($host, $session, $event, $store, $remote, $message);
        $crate::__impl_child_prompt!($host, $session, $event, $registry, $get);
        $crate::__impl_child_runtime!($host, $session, $event, $first, $finish);
        $crate::__impl_child_startup!($host, $session, $store, $prepare, $ensure, $open, $announce);
    };
}
