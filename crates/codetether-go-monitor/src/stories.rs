//! Append story status from the pipeline PRD when it can be read.

use crate::ActiveGoRun;
use serde_json::Value;
pub(crate) fn append(output: &mut String, run: &ActiveGoRun) {
    if let Ok(prd_content) = std::fs::read_to_string(&run.prd_filename)
        && let Ok(prd) = serde_json::from_str::<Value>(&prd_content)
        && let Some(stories) = prd.get("user_stories").and_then(|s| s.as_array())
    {
        output.push_str("\n## Stories\n\n");
        let mut passed_count = 0;
        for story in stories {
            let id = story.get("id").and_then(|v| v.as_str()).unwrap_or("?");
            let title = story.get("title").and_then(|v| v.as_str()).unwrap_or("?");
            let passes = story
                .get("passes")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let icon = if passes {
                passed_count += 1;
                "✅"
            } else {
                "⏳"
            };
            output.push_str(&format!("- {icon} **{id}**: {title}\n"));
        }
        output.push_str(&format!(
            "\n**Progress:** {passed_count}/{} stories passed\n",
            stories.len()
        ));
    }
}
