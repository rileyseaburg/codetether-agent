use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

pub(super) fn enter(project: Option<PathBuf>, allow_network: bool) -> Result<()> {
    if allow_network {
        unsafe {
            std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "1");
        }
    }
    if let Some(project) = project {
        enter_project(&project)?;
    }
    Ok(())
}

fn enter_project(project: &Path) -> Result<()> {
    validate_project(project)?;
    std::env::set_current_dir(project).map_err(|err| {
        anyhow::anyhow!(
            "failed to enter project directory {}: {err}",
            project.display()
        )
    })
}

fn validate_project(project: &Path) -> Result<()> {
    if !project.exists() {
        bail!(
            "project directory does not exist: {}\n\
             hint: `tui` takes an optional path to an existing workspace, \
             not a subcommand. Run `codetether tui` from inside your project, \
             or pass a directory that already exists.",
            project.display()
        );
    }
    if !project.is_dir() {
        bail!("project path is not a directory: {}", project.display());
    }
    Ok(())
}
