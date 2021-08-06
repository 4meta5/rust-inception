use std::fs::{self, File};
use std::io::Write;

fn main() {
    // Paths
    let directory = "test";
    let code_subdirectory = format!("{}/src", directory);
    let cargo_toml_path = format!("{}/Cargo.toml", directory);
    let code_file_path = format!("{}/main.rs", code_subdirectory);
    // Clean existing directory if they exist
    fs::remove_dir_all(code_subdirectory.clone()).ok();
    fs::remove_dir_all(directory).ok();
    // Create folders
    fs::create_dir_all(directory).expect("failed to create test/ directory");
    fs::create_dir_all(code_subdirectory).expect("failed to create test/src directory");
    // Cargo toml text
    let cargo_toml_txt = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
authors = ["Parent Directory"]
edition = "2018"

[dependencies]
        "#,
        directory
    );
    // Code in this file, TODO: limit depth so not infinite
    let code = include_str!("./main.rs");
    // Create files
    let mut cargo_toml_file = File::create(cargo_toml_path).expect("failed to create file");
    let mut code_file = File::create(code_file_path).expect("failed to create file");
    // Write to files
    cargo_toml_file
        .write_all(cargo_toml_txt.as_bytes())
        .expect("failed to write data to share file");
    code_file
        .write_all(code.as_bytes())
        .expect("failed to write data to share file");
    // Run cargo run command in new directory
    std::process::Command::new("cargo")
        .arg("run")
        .current_dir(directory)
        .status()
        .expect("failed to run command");
}
