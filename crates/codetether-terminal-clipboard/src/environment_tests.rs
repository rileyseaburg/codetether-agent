//! Linux headless clipboard detection tests.

#[cfg(target_os = "linux")]
static ENV: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(target_os = "linux")]
#[test]
fn tmux_without_display_is_headless() {
    let _lock = ENV
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let display = std::env::var_os("DISPLAY");
    let wayland = std::env::var_os("WAYLAND_DISPLAY");
    unsafe {
        std::env::remove_var("DISPLAY");
        std::env::remove_var("WAYLAND_DISPLAY");
    }
    assert!(super::is_headless_session());
    restore("DISPLAY", display);
    restore("WAYLAND_DISPLAY", wayland);
}

#[cfg(target_os = "linux")]
fn restore(name: &str, value: Option<std::ffi::OsString>) {
    unsafe {
        match value {
            Some(value) => std::env::set_var(name, value),
            None => std::env::remove_var(name),
        }
    }
}
