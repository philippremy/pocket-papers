use std::{error::Error, path::{Path, PathBuf}, process::{exit, Command}};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {

    // Copy resources
    let resource_dir = PathBuf::from(std::env::var("OUT_DIR")?).join("Resources");
    if !std::fs::exists(&resource_dir)? {
        std::fs::create_dir_all(&resource_dir)?;
    }
    copy_dir_all(concat!(std::env!("CARGO_MANIFEST_DIR"), "/typst_files"), &resource_dir)?;

    // Compile typst-cli
    let target_dir = PathBuf::from(std::env::var("OUT_DIR")?)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    println!("cargo::warning={}", "Build typst CLI...");

    Command::new("cargo")
        .current_dir(
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?)
                .parent()
                .unwrap()
                .join("external")
                .join("typst")
        )
        .args([
            "build",
            "--target",
            &std::env::var("TARGET")?,
            "--artifact-dir",
            target_dir.as_os_str().to_str().unwrap(),
            "-Z",
            "unstable-options"
        ])
        .env("PROFILE", &format!("{}", &std::env::var("PROFILE").map(|var| {
            if &var == "debug" {
                return "".into()
            } else {
                return var
            }
        }).unwrap_or(String::from(""))))
        .output().inspect(|ok| {
            if !ok.status.success() {
                println!("cargo::warning=Cargo did not exit successfully (STDOUT): {}.\n\n", String::from_utf8_lossy(&ok.stdout));
                println!("cargo::warning=Cargo did not exit successfully (STDERR): {}.\n\n", String::from_utf8_lossy(&ok.stderr));
                exit(ok.status.code().unwrap_or(-1));
            }
        })?;

        println!("cargo::warning=Put typst CLI to: {}.", target_dir.as_os_str().to_str().unwrap());

    Ok(())
}