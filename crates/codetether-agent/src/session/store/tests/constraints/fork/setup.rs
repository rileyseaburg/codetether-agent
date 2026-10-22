//! Parent with a correction near the end of a saturated transcript.
use super::{Session, WINDOW, message};
use codetether_context_pin::PinSource;
use codetether_context_policy::{pages::PageKind, state_header::ConstraintCorrection};

pub(super) async fn session() -> Session {
    let mut session = Session::new().await.unwrap();
    for _ in 0..WINDOW {
        session.add_message(message("earlier"));
    }
    session.add_message(message("Keep evidence. Use red."));
    session.add_message(message("Use blue."));
    session.add_message(message("Continue"));
    session.pages[WINDOW] = PageKind::Constraint;
    session.pages[WINDOW + 1] = PageKind::Constraint;
    let old = session.constraint_source(WINDOW).unwrap();
    let new = session.constraint_source(WINDOW + 1).unwrap();
    session.set_corrections(vec![ConstraintCorrection {
        source_turn: WINDOW,
        source_hash: old.text_hash,
        start: 15,
        end: 23,
        replacement_turn: WINDOW + 1,
        replacement_hash: new.text_hash,
    }]);
    session
}
