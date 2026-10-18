use crate::session::Session;
use crate::tui::app::state::App;

#[tokio::test]
async fn status_command_posts_policy_status() {
    let cwd = std::env::current_dir().expect("cwd");
    let mut app = App::default();
    app.state.composer.input = "/status".to_string();
    let mut session = Session::new().await.expect("session");

    assert!(super::run(&mut app, &cwd, &mut session, None, "/status").await);

    let message = app.state.messages.last().expect("status message");
    assert!(message.content.contains("Status"));
    assert!(message.content.contains("Trust:"));
    assert!(app.state.composer.input.is_empty());
}

#[tokio::test]
async fn review_command_leaves_editable_prompt() {
    let cwd = std::env::current_dir().expect("cwd");
    let mut app = App::default();
    let mut session = Session::new().await.expect("session");

    assert!(super::run(&mut app, &cwd, &mut session, None, "/review").await);

    assert!(app.state.composer.input.starts_with("Review the current"));
    assert_eq!(
        app.state.composer.cursor,
        app.state.composer.input.chars().count()
    );
}
