//! A saturated native session whose original sources are outside the tail.
use super::{Session, WINDOW, message};
use codetether_context_policy::pages::PageKind;
pub(super) async fn session() -> Session {
    let mut session = Session::new().await.unwrap();
    session.add_message(message("Keep evidence. Use red. Never deploy."));
    session.add_message(message("Use blue."));
    session.pages[0] = PageKind::Constraint;
    for _ in 0..WINDOW {
        session.add_message(message("later"));
    }
    session
}
