//! Inventory contract for approval-bound no-redirect HTTP clients.

#[test]
fn every_registered_http_surface_disables_redirects() {
    let policy_sources = [
        include_str!("application_mcp.rs"),
        include_str!("image_generation/client.rs"),
        include_str!("voice_input/client.rs"),
        include_str!("webfetch.rs"),
        include_str!("websearch_client.rs"),
        include_str!("image_remote.rs"),
        include_str!("../../crates/codetether-browser/src/browser/session/native/fetch.rs"),
        include_str!(
            "../../crates/codetether-browser/src/browser/session/native/net/http/send.rs"
        ),
        include_str!("../../crates/codetether-browser/src/browser/offline/record.rs"),
        include_str!("../../crates/codetether-browser/src/browser/offline/explain_cors.rs"),
    ];
    for source in policy_sources {
        assert!(source.contains("redirect(reqwest::redirect::Policy::none())"));
    }
    for source in [
        include_str!("voice.rs"),
        include_str!("voice_stream.rs"),
        include_str!("podcast.rs"),
        include_str!("avatar.rs"),
        include_str!("youtube.rs"),
        include_str!("morph_backend.rs"),
    ] {
        assert!(source.contains("no_redirect_client"));
    }
}