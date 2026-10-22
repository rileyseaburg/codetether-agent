//! Native fixture retaining complete sources outside the active window.
use super::{Session, WINDOW, message};
use codetether_context_pin::PinSource;
use codetether_context_policy::{pages::PageKind, state_header::ConstraintCorrection};

pub(super) async fn session() -> Session {
    let mut session = Session::new().await.unwrap();
    session
        .messages
        .push(message("Keep evidence. Use red. Do not deploy."));
    session.messages.push(message("Use blue."));
    session.pages = vec![PageKind::Constraint; 2].into();
    let source = session.constraint_source(0).unwrap();
    let target = session.constraint_source(1).unwrap();
    let start = source.text.find("Use red.").unwrap();
    session.set_corrections(vec![ConstraintCorrection {
        source_turn: 0,
        source_hash: source.text_hash,
        start,
        end: start + "Use red.".len(),
        replacement_turn: 1,
        replacement_hash: target.text_hash,
    }]);
    for _ in 0..WINDOW {
        session.add_message(message("later"));
    }
    session
}
