//! Safe keyboard choices: Enter defaults No and modifiers cannot accept.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[test]
fn answer_review_choices_require_explicit_yes() {
    let mut yes = false;
    let key = |code| KeyEvent::new(code, KeyModifiers::NONE);
    assert_eq!(super::choose(&mut yes, key(KeyCode::Enter)), Some(false));
    assert_eq!(super::choose(&mut yes, key(KeyCode::Right)), None);
    assert!(yes);
    assert_eq!(super::choose(&mut yes, key(KeyCode::Enter)), Some(true));
    assert_eq!(super::choose(&mut yes, key(KeyCode::Esc)), Some(false));
    assert_eq!(
        super::choose(&mut yes, key(KeyCode::Char('n'))),
        Some(false)
    );
    let modified = KeyEvent::new(KeyCode::Char('y'), KeyModifiers::ALT);
    assert_eq!(super::choose(&mut yes, modified), None);
}
