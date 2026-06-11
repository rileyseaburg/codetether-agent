use crate::cli::ProjectCommand;
use crate::config::ProjectTrustStore;
use anyhow::Result;

pub fn execute(command: ProjectCommand) -> Result<()> {
    let store = ProjectTrustStore::for_current_workspace()?;
    match command {
        ProjectCommand::Status => print_status(&store),
        ProjectCommand::Trust => {
            store.trust()?;
            print_status(&store);
        }
        ProjectCommand::Untrust => {
            store.untrust()?;
            print_status(&store);
        }
    }
    Ok(())
}

fn print_status(store: &ProjectTrustStore) {
    let status = store.status();
    let trust = if status.trusted {
        "trusted"
    } else {
        "untrusted"
    };
    println!("Project: {}", status.workspace.display());
    println!("Trust: {trust}");
    println!("Key: {}", status.key);
    println!("Record: {}", status.record_path.display());
}
