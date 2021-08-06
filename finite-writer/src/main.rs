mod depth;
use depth::DEPTH;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    // Paths
    let directory = "test";
    let code_subdirectory = format!("{}/src", directory);
    let cargo_toml_path = format!("{}/Cargo.toml", directory);
    let main_rs_path = format!("{}/main.rs", code_subdirectory);
    let depth_rs_path = format!("{}/depth.rs", code_subdirectory);
    // Remove existing directory if exists
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
    let (main_code, new_depth) = if DEPTH > 1 {
        (include_str!("./main.rs"), DEPTH - 1)
    } else {
        (
            r#"fn main() { println!("GOODBYE WORLD, NO MORE WRITING") }"#,
            0u8,
        )
    };
    println!("NEW DEPTH IS {}", new_depth);
    // Create files
    let mut cargo_toml = File::create(cargo_toml_path).expect("failed to create Cargo.toml");
    let mut main_rs = File::create(main_rs_path).expect("failed to create main.rs");
    // Write to files
    cargo_toml
        .write_all(cargo_toml_txt.as_bytes())
        .expect("failed to write to Cargo.toml");
    main_rs
        .write_all(main_code.as_bytes())
        .expect("failed to write to main.rs");
    // Write to depth.rs iff DEPTH > 0
    if DEPTH > 0 {
        let depth_code = format!(
            r#"/// Used to bound writing depth, decrements
pub const DEPTH: u8 = {};
            "#,
            new_depth
        );
        let mut depth_rs = File::create(depth_rs_path).expect("failed to create depth.rs");
        depth_rs
            .write_all(depth_code.as_bytes())
            .expect("failed to write to depth.rs");
    }
    // Run cargo run command in new directory
    std::process::Command::new("cargo")
        .arg("run")
        .current_dir(directory)
        .status()
        .expect("failed to run `cargo run` command");
}
