use std::env;
use std::fs;
use std::path::Path;

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&path, &dst_path)?;
        } else if file_type.is_file() {
            fs::copy(&path, &dst_path)?;
        }
    }

    Ok(())
}

fn main() {
    let profile = env::var("PROFILE").unwrap();
    
    // Determine the target directory based on profile
    let target_dir = if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        Path::new(&target).join(&profile)
    } else {
        // Default cargo target directory structure
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&manifest_dir)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("target")
            .join(&profile)
    };

    println!("cargo:rerun-if-changed=config.json");
    println!("cargo:rerun-if-changed=orders");

    // Copy config.json to target directory
    let config_src = Path::new("config.json");
    if config_src.exists() {
        let config_dst = target_dir.join("config.json");
        if config_dst.exists() {
            println!(
                "cargo:warning=Skipped copying config.json because destination already exists: {}",
                config_dst.display()
            );
        } else if let Err(e) = fs::copy(&config_src, &config_dst) {
            println!("cargo:warning=Failed to copy config.json: {}", e);
        } else {
            println!("cargo:warning=Copied config.json to {}", config_dst.display());
        }
    }

    // Copy orders directory to target directory
    let orders_src = Path::new("orders");
    if orders_src.exists() && orders_src.is_dir() {
        let orders_dst = target_dir.join("orders");
        if let Err(e) = copy_dir_recursive(orders_src, &orders_dst) {
            println!(
                "cargo:warning=Failed to copy orders directory {} -> {}: {}",
                orders_src.display(),
                orders_dst.display(),
                e
            );
        } else {
            println!(
                "cargo:warning=Copied orders directory {} -> {}",
                orders_src.display(),
                orders_dst.display()
            );
        }
    }
}


