use std::env;
use std::process::Command;

// Passes the version of the compiler to the application, which shows it as
// its runtime: "rustc 1.100.0 (...)" becomes RUSTC_VERSION=1.100.0.
fn main() {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let output = Command::new(rustc)
        .arg("--version")
        .output()
        .expect("failed to run rustc --version");
    let version = String::from_utf8_lossy(&output.stdout);
    let version = version.split_whitespace().nth(1).unwrap_or("unknown");

    println!("cargo::rustc-env=RUSTC_VERSION={version}");
    println!("cargo::rerun-if-changed=build.rs");
}
