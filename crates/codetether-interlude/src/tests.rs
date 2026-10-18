use std::time::Duration;

use super::InterludeState;

#[test]
fn movement_is_bounded_and_matching_a_star_scores() {
    let mut game = InterludeState::new();
    for _ in 0..9 {
        game.left();
    }
    assert_eq!(game.lane(), 0);
    game.right();
    game.advance_to(Duration::from_millis(1_500));
    assert_eq!(game.score(), 1);
}
