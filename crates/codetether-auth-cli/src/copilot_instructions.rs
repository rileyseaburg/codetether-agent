//! GitHub device challenge display.
use codetether_device_auth::DeviceCodeResponse;

pub(super) fn show(device: &DeviceCodeResponse) {
    println!("GitHub Copilot device authentication");
    println!(
        "Open this URL: {}",
        device
            .verification_uri_complete
            .as_deref()
            .unwrap_or(&device.verification_uri)
    );
    println!("Enter code: {}", device.user_code);
    println!("Waiting for authorization...");
}
