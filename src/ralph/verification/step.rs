use super::{file, shell, url};
use crate::ralph::types::VerificationStep;
use anyhow::Result;
use std::path::Path;

pub async fn run(root: &Path, step: &VerificationStep) -> Result<()> {
    match step {
        VerificationStep::Shell {
            command,
            cwd,
            expect_output_contains,
            expect_files_glob,
            ..
        } => shell::run(
            root,
            command,
            cwd,
            expect_output_contains,
            expect_files_glob,
        ),
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
