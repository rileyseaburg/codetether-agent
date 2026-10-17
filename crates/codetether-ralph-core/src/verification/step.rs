//! Dispatch verification variants to shell, file or URL checks.
use super::ShellExecutor;
use super::{file, shell, url};
use crate::VerificationStep;
use anyhow::Result;
use std::path::Path;

pub async fn run(
    root: &Path,
    step: &VerificationStep,
    client: &reqwest::Client,
    executor: &dyn ShellExecutor,
) -> Result<()> {
    match step {
        VerificationStep::Shell {
            command,
            cwd,
            expect_output_contains,
            expect_files_glob,
            ..
        } => {
            shell::run(
                root,
                command,
                cwd,
                expect_output_contains,
                expect_files_glob,
                executor,
            )
            .await
        }
        VerificationStep::FileExists { path, glob, .. } => file::exists(root, path, *glob),
        VerificationStep::Url {
            url: target,
            method,
            expect_status,
            expect_body_contains,
            timeout_secs,
            ..
        } => {
            url::check(
                client,
                target,
                method,
                *expect_status,
                expect_body_contains,
                *timeout_secs,
            )
            .await
        }
    }
}
