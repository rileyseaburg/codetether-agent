fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-env-changed=PROTOC");
    // If PROTOC isn't set, try common locations
    if std::env::var("PROTOC").is_err() {
        let home = std::env::var("HOME").unwrap_or_default();
        let local_protoc = format!("{home}/.local/bin/protoc");
        if std::path::Path::new(&local_protoc).exists() {
            // SAFETY: build scripts are single-threaded
            unsafe {
                std::env::set_var("PROTOC", &local_protoc);
            }
        }
    }

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &["proto/a2a/v1/a2a.proto"],
            &[
                "proto",
                &format!(
                    "{}/.local/include",
                    std::env::var("HOME").unwrap_or_default()
                ),
            ],
        )?;
    Ok(())
}
