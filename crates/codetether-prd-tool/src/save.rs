//! save for PRD construction and validation.
use super::*;
impl PrdTool {
    pub(super) async fn save(&self, p: Params) -> Result<ToolResult> {
        let project = p.project.unwrap_or_else(|| "Project".to_string());
        let feature = p.feature.unwrap_or_else(|| "Feature".to_string());
        let prd_path = PathBuf::from(p.prd_path.unwrap_or_else(|| "prd.json".to_string()));

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
                "At least one story is required for save",
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

        prd.save(&prd_path).await.context("Failed to save PRD")?;
        Ok(ToolResult::success(format!(
            "PRD saved to: {}",
            prd_path.display()
        )))
    }
}
