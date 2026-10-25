//! Credential-free output proof for the opt-in live test.
use codetether_tool_core::ToolResult;
use serde_json::json;
mod state;

pub(super) fn verify(first: &ToolResult, second: &ToolResult) {
    state::verify(first, second);
    let paths: Vec<_> = [first, second]
        .into_iter()
        .map(|result| result.metadata["saved_path"].as_str().unwrap())
        .collect();
    for path in &paths {
        assert!(std::fs::metadata(path).unwrap().len() > 1024);
    }
    assert_ne!(
        std::fs::read(paths[0]).unwrap(),
        std::fs::read(paths[1]).unwrap()
    );
    let report = json!({
        "level":"static/local native Rust tool against live Google HTTP",
        "same_conversation":true, "fresh_response":true, "images":paths,
        "first_mime":first.metadata["image_data_url"]["mime_type"],
        "second_mime":second.metadata["image_data_url"]["mime_type"]
    });
    if let Some(directory) = std::env::var_os("GEMINI_RUST_EVIDENCE_DIR") {
        std::fs::create_dir_all(&directory).unwrap();
        let path = std::path::PathBuf::from(directory).join("summary.json");
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    println!("{report}");
}
