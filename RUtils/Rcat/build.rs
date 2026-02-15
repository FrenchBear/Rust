// Build script extracting dependencies versions at build time, add them to environment, so they can
// be retrieved at compile-time in main app with env!() macro
//
// 2025-07-05   PV      First version, with the help of Gemini
// 2025-10-22   PV      Clippy review
// 2026-02-15   PV      Added icon, file description and copyright

use std::env;
use std::fs;
use std::path::PathBuf;
use toml::Value;

fn main() {
    // Locate and read Cargo.lock
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lock_path = PathBuf::from(manifest_dir).join("Cargo.lock");
    let lock_content = fs::read_to_string(lock_path).expect("Failed to read Cargo.lock");

    // Parse the TOML data
    let lockfile: toml::Value = toml::from_str(&lock_content).expect("Failed to parse Cargo.lock");

    // Find the package entry
    let packages = lockfile.get("package").and_then(|p| p.as_array()).expect("Could not find [[package]] in Cargo.lock");

    generate_variable(packages, "getopts");

    // Tell cargo to re-run the build script if Cargo.lock changes.
    println!("cargo:rerun-if-changed=Cargo.lock");

    // Retrieve copyright from cargo.toml section [package.metadata]
    let manifest_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/Cargo.toml";
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(manifest_path)
        .no_deps() // We only care about our own crate
        .exec()
        .unwrap();

    // Access the [package.metadata] JSON object
    let root_package = metadata.root_package().unwrap();
    let copyright = root_package.metadata["copyright"].as_str().unwrap_or_default();

    // [package] info can be accessed directly
    let description = env!("CARGO_PKG_DESCRIPTION");
    let name = env!("CARGO_PKG_NAME");

    // Include an icon
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("cat.ico");
        res.set("FileDescription", format!("{name}: {description}").as_str());
        res.set("LegalCopyright", copyright);
        res.compile().unwrap();
    }
}

fn generate_variable(packages: &[Value], dependency_name: &'static str) {
    let dep_package = packages.iter().find(|p| {
        p.get("name").and_then(|n| n.as_str()) == Some(dependency_name)
    }).unwrap_or_else(|| panic!("Could not find '{}' in Cargo.lock's [[package]] list", dependency_name));

    // Extract the version
    let version = dep_package.get("version")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| panic!("Could not find version for '{}' in Cargo.lock", dependency_name));

    // Expose the version as a new environment variable
    // This `cargo:rustc-env` instruction tells Cargo to set the `DEP_XXXX_VERSION`
    // environment variable for the compilation of the main crate.
    println!("cargo:rustc-env=DEP_{}_VERSION={}", dependency_name.to_uppercase().replace('-', "_"), version);
}
