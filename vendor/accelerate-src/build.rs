fn main() {
    if std::env::var("CARGO_CFG_TARGET_VENDOR").ok().as_deref() == Some("apple") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
}
