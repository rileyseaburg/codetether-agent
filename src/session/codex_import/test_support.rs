use std::path::{Path, PathBuf};

pub(super) fn write_codex_fixture(base: &Path, workspace: &Path) -> PathBuf {
    let codex_home = base.join(".codex");
    let session_dir = codex_home.join("sessions/2026/03/26");
    std::fs::create_dir_all(&session_dir).expect("session dir");
    std::fs::write(
        codex_home.join("session_index.jsonl"),
        "{\"id\":\"019d2acd-8b3f-70e0-b019-854d52272660\",\"thread_name\":\"Imported from Codex\",\"updated_at\":\"2026-03-26T15:41:16.821Z\"}\n",
    )
    .expect("index");
    let path =
        session_dir.join("rollout-2026-03-26T15-40-06-019d2acd-8b3f-70e0-b019-854d52272660.jsonl");
    std::fs::write(&path, fixture_body(workspace)).expect("session file");
    path
}

fn fixture_body(workspace: &Path) -> String {
    let workspace = workspace.display().to_string();
    concat!(
        "{\"timestamp\":\"2026-03-26T15:40:21.283Z\",\"type\":\"session_meta\",\"payload\":{\"id\":\"019d2acd-8b3f-70e0-b019-854d52272660\",\"timestamp\":\"2026-03-26T15:40:06.884Z\",\"cwd\":\"__WORKSPACE__\"}}\n",
        "{\"timestamp\":\"2026-03-26T15:40:21.470Z\",\"type\":\"turn_context\",\"payload\":{\"model\":\"gpt-5.4\"}}\n",
        "{\"timestamp\":\"2026-03-26T15:40:22.000Z\",\"type\":\"response_item\",\"payload\":{\"type\":\"message\",\"role\":\"developer\",\"content\":[{\"type\":\"input_text\",\"text\":\"skip\"}]}}\n",
        "{\"timestamp\":\"2026-03-26T15:40:23.000Z\",\"type\":\"response_item\",\"payload\":{\"type\":\"message\",\"role\":\"user\",\"content\":[{\"type\":\"input_text\",\"text\":\"Import this session please\"},{\"type\":\"input_image\",\"image_url\":\"data:image/png;base64,abc\"}]}}\n",
        "{\"timestamp\":\"2026-03-26T15:40:24.000Z\",\"type\":\"response_item\",\"payload\":{\"type\":\"reasoning\",\"summary\":[\"checked files\"],\"content\":\"thinking text\"}}\n",
        "{\"timestamp\":\"2026-03-26T15:40:25.000Z\",\"type\":\"response_item\",\"payload\":{\"type\":\"function_call\",\"name\":\"exec_command\",\"arguments\":\"{\\\"cmd\\\":\\\"pwd\\\"}\",\"call_id\":\"call_1\"}}\n",
        "{\"timestamp\":\"2026-03-26T15:40:26.000Z\",\"type\":\"response_item\",\"payload\":{\"type\":\"function_call_output\",\"call_id\":\"call_1\",\"output\":\"pwd output\"}}\n",
        "{\"timestamp\":\"2026-03-26T15:40:27.000Z\",\"type\":\"response_item\",\"payload\":{\"type\":\"message\",\"role\":\"assistant\",\"content\":[{\"type\":\"output_text\",\"text\":\"done\"}]}}\n",
        "{\"timestamp\":\"2026-03-26T15:40:28.000Z\",\"type\":\"event_msg\",\"payload\":{\"type\":\"token_count\",\"info\":{\"total_token_usage\":{\"input_tokens\":10,\"cached_input_tokens\":2,\"output_tokens\":5,\"total_tokens\":15}}}}\n"
    )
    .replace("__WORKSPACE__", &workspace)
}
