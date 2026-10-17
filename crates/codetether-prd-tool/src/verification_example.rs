//! Example verification payload shown by task analysis.
pub(super) const VERIFICATION_EXAMPLE_JSON: &str = r#"{
  \"verification_steps\": [
    {
      \"type\": \"shell\",
      \"name\": \"cypress_e2e\",
      \"command\": \"npx cypress run\",
      \"expect_files_glob\": [\"cypress/videos/**/*.mp4\"]
    },
    {
      \"type\": \"url\",
      \"name\": \"deployment_live\",
      \"url\": \"https://example.com/health\",
      \"expect_status\": 200,
      \"expect_body_contains\": [\"version:1.2.3\"],
      \"timeout_secs\": 30
    }
  ]
}"#;
