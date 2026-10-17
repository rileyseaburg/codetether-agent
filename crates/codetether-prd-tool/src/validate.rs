//! validate for PRD construction and validation.
use super::*;
impl PrdTool {
    pub(super) async fn validate(&self, p: Params) -> Result<ToolResult> {
        let prd: Prd = if let Some(json_str) = p.prd_json {
            serde_json::from_str(&json_str).context("Failed to parse prd_json - invalid JSON")?
        } else if let Some(path) = &p.prd_path {
            let prd_path = PathBuf::from(path);
            if !prd_path.exists() {
                return Ok(ToolResult::error(format!("PRD file not found: {path}")));
            }
            let content = tokio::fs::read_to_string(&prd_path).await?;
            serde_json::from_str(&content).context("Failed to parse PRD file - invalid JSON")?
        } else if let Some(stories) = p.stories {
            let project = p.project.clone().unwrap_or_else(|| "Project".to_string());
            let feature = p.feature.clone().unwrap_or_else(|| "Feature".to_string());

            let stories: Vec<UserStory> = stories.into_iter().map(convert::story).collect();

            Prd {
                project,
                feature,
                branch_name: String::new(),
                version: "1.0".to_string(),
                user_stories: stories,
                technical_requirements: Vec::new(),
                quality_checks: QualityChecks::default(),
                created_at: String::new(),
                updated_at: String::new(),
            }
        } else {
            return Ok(ToolResult::structured_error(
                "missing_field",
                self.id(),
                "validate requires one of: prd_json, prd_path, or stories",
                Some(vec!["prd_json|prd_path|stories"]),
                None,
            ));
        };

        validation_response::render(&prd)
    }
}
