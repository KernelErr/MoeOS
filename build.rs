use chrono::prelude::*;

fn main() {
    let utc = Utc::now().to_rfc2822();
    println!("cargo:rustc-env=BUILD_NAME={}", "MoeOS");
    println!("cargo:rustc-env=BUILD_VERSION={}", "0.1.0");
    println!("cargo:rustc-env=BUILD_TIME={}", utc);
}