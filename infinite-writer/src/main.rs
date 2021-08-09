mod height;
use height::HEIGHT;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    // Paths
    let directory = "test";
    let code_subdirectory = format!("{}/src", directory);
    let cargo_toml_path = format!("{}/Cargo.toml", directory);
    let main_rs_path = format!("{}/main.rs", code_subdirectory);
    let height_rs_path = format!("{}/height.rs", code_subdirectory);
    // Clean existing directory if exists
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
    // Code in this file
    let main_rs_code = include_str!("./main.rs");
    let new_height = HEIGHT + 1u8;
    println!("NEW HEIGHT IS {}", new_height);
    let height_rs_code = format!(
        r#"/// Used to measure height, increments
pub const HEIGHT: u8 = {};
        "#,
        new_height
    );
    // Create files
    let mut cargo_toml = File::create(cargo_toml_path).expect("failed to create Cargo.toml");
    let mut main_rs = File::create(main_rs_path).expect("failed to create main.rs");
    let mut height_rs = File::create(height_rs_path).expect("failed to create height.rs");
    // Write to files
    cargo_toml
        .write_all(cargo_toml_txt.as_bytes())
        .expect("failed to write to Cargo.toml");
    main_rs
        .write_all(main_rs_code.as_bytes())
        .expect("failed to write to main.rs");
    height_rs
        .write_all(height_rs_code.as_bytes())
        .expect("failed to write to height.rs");
    // Run cargo run command in new directory
    std::process::Command::new("cargo")
        .arg("run")
        .current_dir(directory)
        .status()
        .expect("failed to run `cargo run` command");
}
