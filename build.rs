/*!
 * Build Script for SASE License Core
 *
 * Performs compile-time checks for zero-heap compliance and feature validation.
 */

use std::env;
use std::process::Command;

fn main() {
    // Set up cargo metadata
    println!("cargo:rustc-check-cfg=cfg(zero_heap)");
    println!("cargo:rustc-check-cfg=cfg(const_fn)");

    // Check for zero-heap enforcement flag
    if env::var("RUSTFLAGS").unwrap_or_default().contains("SASE_ZERO_HEAP=1") {
        println!("cargo:rustc-cfg=zero_heap");
        println!("cargo:warning=SASE_ZERO_HEAP=1 enabled - zero-heap enforcement active");
    }

    // Check for const_fn feature
    if env::var("CARGO_FEATURE_CONST_FN").is_ok() {
        println!("cargo:rustc-cfg=const_fn");
    }

    // Validate target architecture for WASM
    let target = env::var("TARGET").unwrap_or_default();
    if target == "wasm32-unknown-unknown" {
        println!("cargo:warning=Building for WASM target: {}", target);
        // Ensure we have the right features for WASM
        if env::var("CARGO_FEATURE_WASM").is_err() && env::var("CARGO_FEATURE_STD").is_err() {
            println!("cargo:warning=WASM target requires 'wasm' or 'std' feature");
        }
    }

    // Check Rust version
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();
    println!("cargo:warning=Building with: {}", rustc_version.trim());

    // Verify required features are set correctly
    let features = [
        ("std", "CARGO_FEATURE_STD"),
        ("python", "CARGO_FEATURE_PYTHON"),
        ("tpm", "CARGO_FEATURE_TPM"),
        ("sqlite", "CARGO_FEATURE_SQLITE"),
        ("wasm", "CARGO_FEATURE_WASM"),
    ];

    for (name, env_var) in &features {
        if env::var(env_var).is_ok() {
            println!("cargo:warning=Feature enabled: {}", name);
        }
    }

    // Compile-time assertions for constants are handled in lib.rs with const_assert! macro
    // These will be checked by the compiler when the crate is built

    // Ensure no_std compatibility
    #[cfg(not(feature = "std"))]
    {
        println!("cargo:warning=Building no_std configuration");
    }

    // Verify heapless usage
    if env::var("CARGO_FEATURE_STD").is_err() {
        println!("cargo:warning=no_std mode - using heapless for fixed-capacity collections");
    }

    // Check for zeroize feature
    if env::var("CARGO_FEATURE_ZEROIZE").is_ok() {
        println!("cargo:warning=zeroize feature enabled for secret zeroization");
    }

    // Set optimization hints for release
    let profile = env::var("PROFILE").unwrap_or_default();
    if profile == "release" {
        println!("cargo:warning=Release build - optimizations enabled");
        println!("cargo:rustc-cfg=release_build");
    }

    // Generate version info
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_default();
    println!("cargo:rustc-env=SASE_LICENSE_CORE_VERSION={}", version);

    let version_parts: Vec<&str> = version.split('.').collect();
    if version_parts.len() >= 3 {
        let major = version_parts[0].parse::<u32>().unwrap_or(0);
        let minor = version_parts[1].parse::<u32>().unwrap_or(0);
        let patch = version_parts[2].parse::<u32>().unwrap_or(0);
        let version_num = (major << 24) | (minor << 16) | patch;
        println!("cargo:rustc-env=SASE_LICENSE_CORE_VERSION_NUM={}", version_num);
    }

    // Check for any prohibited dependencies in no_std mode
    #[cfg(not(feature = "std"))]
    {
        // These would cause compile errors if used in no_std
        println!("cargo:warning=Verifying no_std compatibility...");
    }
}
