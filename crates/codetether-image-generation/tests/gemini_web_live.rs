//! Opt-in native Rust image tool proof; never runs in the default test suite.
use codetether_image_generation::ImageGenerationTool;
use codetether_tool_core::Tool;
use serde_json::json;
mod gemini_live_support;

#[tokio::test]
#[ignore = "requires an explicit signed-in Gemini cookie file and live network"]
async fn gemini_web_generates_and_refines() {
    assert!(
        std::env::var_os("GEMINI_WEB_COOKIE_FILE").is_some(),
        "Set GEMINI_WEB_COOKIE_FILE for this live test"
    );
    let tool = ImageGenerationTool::new();
    let first = tool.execute(json!({
        "provider":"gemini-web",
        "prompt":"Generate a clean product photograph of a green ceramic teapot on a pale wooden table by a sunlit window, with a folded cream napkin on the left. No text or logos."
    })).await.unwrap();
    assert!(first.success, "{}", first.output);
    let state_path = first.metadata["refinement_path"]
        .as_str()
        .expect("refinable result");
    let second = tool.execute(json!({
        "provider":"gemini-web", "refine_from":state_path,
        "prompt":"Edit that same image: change only the teapot glaze from green to purple. Keep the same shape, position, table, napkin, window, lighting and composition. Return the edited image."
    })).await.unwrap();
    assert!(second.success, "{}", second.output);
    gemini_live_support::verify(&first, &second);
}
