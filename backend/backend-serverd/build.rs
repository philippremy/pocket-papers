use std::{error::Error, path::PathBuf, process::{exit, Command}};

fn main() -> Result<(), Box<dyn Error>> {

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
            "--release",
            "--target",
            &std::env::var("TARGET")?,
            "--artifact-dir",
            target_dir.as_os_str().to_str().unwrap(),
            "-Z",
            "unstable-options"
        ])
        .envs([
            ("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER", "/usr/local/bin/ld.mold"),
            ("CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_LINKER", "/usr/local/bin/ld.mold"),
            ("CARGO_TARGET_x86_64_UNKNOWN_LINUX_GNU_LINKER", "/usr/local/bin/ld.mold"),
            ("CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER", "/usr/local/bin/ld.mold"),
        ])
        .output().inspect(|ok| {
            if !ok.status.success() {
                println!("cargo::warning=Cargo did not exit successfully (STDOUT): {}.\n\n", String::from_utf8_lossy(&ok.stdout));
                println!("cargo::warning=Cargo did not exit successfully (STDERR): {}.\n\n", String::from_utf8_lossy(&ok.stderr));
                exit(ok.status.code().unwrap_or(-1));
            }
        })?;

        println!("cargo::warning=Put typst CLI to: {}.", target_dir.as_os_str().to_str().unwrap());

        println!("cargo::warning={}", "Build rust-rpxy...");

        Command::new("cargo")
            .current_dir(
                PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?)
                    .parent()
                    .unwrap()
                    .join("external")
                    .join("rust-rpxy")
            )
            .args([
                "build",
                "--release",
                "--target",
                &std::env::var("TARGET")?,
                "--artifact-dir",
                target_dir.as_os_str().to_str().unwrap(),
                "-Z",
                "unstable-options"
            ])
            .envs([
                ("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER", "/usr/local/bin/mold"),
                ("CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_LINKER", "/usr/local/bin/mold"),
                ("CARGO_TARGET_x86_64_UNKNOWN_LINUX_GNU_LINKER", "/usr/local/bin/mold"),
                ("CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER", "/usr/local/bin/mold"),
            ])
            .output().inspect(|ok| {
                if !ok.status.success() {
                    println!("cargo::warning=Cargo did not exit successfully (STDOUT): {}.\n\n", String::from_utf8_lossy(&ok.stdout));
                    println!("cargo::warning=Cargo did not exit successfully (STDERR): {}.\n\n", String::from_utf8_lossy(&ok.stderr));
                    exit(ok.status.code().unwrap_or(-1));
                }
            })?;
    
            println!("cargo::warning=Put rust-rpxy to: {}.", target_dir.as_os_str().to_str().unwrap());

        println!("cargo::warning={}", "Build backend-server...");

        Command::new("cargo")
            .current_dir(
                PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?)
                    .parent()
                    .unwrap()
                    .join("backend-server")
            )
            .args([
                "build",
                "--release",
                "--target",
                &std::env::var("TARGET")?,
                "--artifact-dir",
                target_dir.as_os_str().to_str().unwrap(),
                "-Z",
                "unstable-options"
            ])
            .envs([
                ("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER", "/usr/local/bin/mold"),
                ("CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_LINKER", "/usr/local/bin/mold"),
                ("CARGO_TARGET_x86_64_UNKNOWN_LINUX_GNU_LINKER", "/usr/local/bin/mold"),
                ("CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER", "/usr/local/bin/mold"),
            ])
            .output().inspect(|ok| {
                if !ok.status.success() {
                    println!("cargo::warning=Cargo did not exit successfully (STDOUT): {}.\n\n", String::from_utf8_lossy(&ok.stdout));
                    println!("cargo::warning=Cargo did not exit successfully (STDERR): {}.\n\n", String::from_utf8_lossy(&ok.stderr));
                    exit(ok.status.code().unwrap_or(-1));
                }
            })?;
    
            println!("cargo::warning=Put backend-server to: {}.", target_dir.as_os_str().to_str().unwrap());

    Ok(())
}