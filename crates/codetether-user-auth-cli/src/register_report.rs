//! Display account registration and provisioning details.
use crate::register_response::RegisterResponse;

pub(super) fn print(reg: &RegisterResponse) {
    println!(
        "Account created for {} (user_id={})",
        reg.email, reg.user_id
    );
    println!("{}", reg.message);
    if let Some(status) = reg.provisioning_status.as_deref() {
        println!("Provisioning status: {}", status);
    }
    if let Some(url) = reg.instance_url.as_deref() {
        println!("Instance URL: {}", url);
    }
    if let Some(ns) = reg.instance_namespace.as_deref() {
        println!("Instance namespace: {}", ns);
    }
}
