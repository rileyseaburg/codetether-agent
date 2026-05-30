use super::*;

#[test]
fn help_overlay_consumes_chat_arrow_navigation() {
    let mut app = App::default();
    app.state.show_help = true;
    app.state.set_view_mode(ViewMode::Chat);
    app.state.set_chat_max_scroll(25);
    app.state.scroll_to_bottom();

    handle_down(&mut app, KeyModifiers::NONE);

    assert_eq!(app.state.help_scroll.offset, 1);
    assert_eq!(app.state.chat_scroll, 1_000_000);
}
