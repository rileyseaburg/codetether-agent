//! Tests for forage offer Y/N answer handling.

use super::*;
use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::forage_run::ForageOffer;

fn app_with_offer() -> App {
    let mut app = App::default();
    app.state.forage.pending_offer = Some(ForageOffer {
        top: 3,
        model: None,
    });
    app
}

#[tokio::test]
async fn no_offer_returns_false() {
    let mut app = App::default();
    let session = Session::new().await.expect("session");
    assert!(!handle_offer_answer(&mut app, &session, "yes"));
}

#[tokio::test]
async fn negative_answer_dismisses_offer() {
    let mut app = app_with_offer();
    let session = Session::new().await.expect("session");
    assert!(handle_offer_answer(&mut app, &session, "no"));
    assert!(app.state.forage.pending_offer.is_none());
    assert!(!app.state.forage.active);
}

#[test]
fn affirmative_variants_are_recognized() {
    for word in ["y", "yes", "yeah", "sure", "ok", "go", "do it", "start"] {
        assert!(is_affirmative(word), "{word} should be affirmative");
    }
    assert!(!is_affirmative("no"));
    assert!(!is_affirmative("maybe later"));
}
