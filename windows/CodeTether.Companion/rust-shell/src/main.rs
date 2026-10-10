//! Native tray companion servicing paired iPhone capture requests in the background.
#![cfg_attr(windows, windows_subsystem = "windows")]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(windows)]
mod native;
#[cfg(windows)]
mod relay;

fn main() -> std::process::ExitCode {
    #[cfg(windows)]
    {
        match native::run() {
            Ok(()) => std::process::ExitCode::SUCCESS,
            Err(error) => {
                native::notice(&error.to_string());
                std::process::ExitCode::FAILURE
            }
        }
    }
    #[cfg(not(windows))]
    {
        eprintln!("CodeTether Screen Companion requires Windows.");
        std::process::ExitCode::FAILURE
    }
}
