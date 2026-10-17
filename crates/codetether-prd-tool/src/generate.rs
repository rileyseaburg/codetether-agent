//! generate for PRD construction and validation.
use super::*;
impl PrdTool {
    pub(super) async fn generate(&self, p: Params) -> Result<ToolResult> {
        let project = p.project.unwrap_or_else(|| "Project".to_string());
        let feature = p.feature.unwrap_or_else(|| "Feature".to_string());

        let stories: Vec<UserStory> = p
            .stories
            .unwrap_or_default()
            .into_iter()
            .map(convert::story)
            .collect();

        if stories.is_empty() {
            return Ok(ToolResult::structured_error(
                "missing_field",
                self.id(),
                "At least one story is required",
                Some(vec!["stories"]),
                None,
            ));
        }

        let quality_checks = quality::convert(p.quality_checks);

        let prd = Prd {
            project: project.clone(),
            feature: feature.clone(),
            branch_name: format!("feature/{}", feature.to_lowercase().replace(' ', "-")),
            version: "1.0".to_string(),
            user_stories: stories,
            technical_requirements: Vec::new(),
            quality_checks,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        let json_str = serde_json::to_string_pretty(&prd)?;
        Ok(
            ToolResult::success(format!("# Generated PRD\n\n```json\n{}\n```\n", json_str))
                .with_metadata("prd", serde_json::to_value(&prd)?),
        )
    }
}
