use chrono::prelude::*;

fn main() {
    let build_time = Utc::now().with_timezone(&chrono::Local).to_rfc2822();
    println!("cargo:rustc-env=BUILD_NAME=MoeOS");
    println!("cargo:rustc-env=BUILD_VERSION=0.1.0");
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
}
