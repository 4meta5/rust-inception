use std::fs::{self, File};
use std::io::Write;

fn main() {
    fs::remove_dir_all("test/src").ok();
    fs::remove_dir_all("test").ok();
    fs::create_dir_all("test").expect("failed to create test/ directory");
    fs::create_dir_all("test/src").expect("failed to create test/src directory");
    let cargo_toml_txt = format!(
        r#"[package]
name = "test"
version = "0.1.0"
authors = ["Amar Singh"]
edition = "2018"

[dependencies]

[workspace]
		"#,
    );
    let mut file = File::create("test/Cargo.toml").expect("failed to create file");
    file.write_all(cargo_toml_txt.as_bytes())
        .expect("failed to write data to share file");
    let mut file = File::create("test/src/main.rs").expect("failed to create file");
    let code = format!(
        r#"
fn main() {{ println!("HELLO WORLD"); }}
        "#,
    );
    file.write_all(code.as_bytes())
        .expect("failed to write data to share file");
    std::process::Command::new("cargo")
        .arg("run")
        .current_dir("test")
        .status()
        .expect("failed to run command");
}
