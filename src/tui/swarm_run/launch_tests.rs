use super::start;
use crate::session::Session;
use crate::tui::app::state::App;

#[tokio::test]
async fn active_swarm_rejects_a_second_launch() {
    let mut app = App::default();
    app.state.swarm.active = true;
    let session = Session::new().await.expect("session");

    assert!(start(&mut app, &session, "another task"));
    assert_eq!(
        app.state.status,
        "A swarm is already running; cancel it before starting another."
    );
}
