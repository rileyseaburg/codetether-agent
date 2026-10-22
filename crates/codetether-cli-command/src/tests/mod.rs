//! Root argument parsing and help contracts with a fixed application identity.
use crate::Command;
use crate::root::ApplicationMetadata;

#[derive(Debug)]
struct TestApplication;
impl ApplicationMetadata for TestApplication {
    const VERSION: &'static str = "1.0.0";
    const ABOUT: &'static str = "CLI parser test application";
}
type Cli = crate::root::Cli<TestApplication>;

mod access_mode;
mod defaults;
mod rust_build;
mod vault_help;
mod windows;
