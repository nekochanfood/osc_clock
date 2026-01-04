use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory (where the binary will be placed)
    // let out_dir = env::var("OUT_DIR").unwrap();
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
        if let Err(e) = fs::copy(&config_src, &config_dst) {
            println!("cargo:warning=Failed to copy config.json: {}", e);
        } else {
            println!("cargo:warning=Copied config.json to {}", config_dst.display());
        }
    }

    // Copy orders directory to target directory
    let orders_src = Path::new("orders");
    if orders_src.exists() && orders_src.is_dir() {
        let orders_dst = target_dir.join("orders");
        
        // Create orders directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(&orders_dst) {
            println!("cargo:warning=Failed to create orders directory: {}", e);
        } else {
            // Copy all JSON files from orders directory
            if let Ok(entries) = fs::read_dir(orders_src) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(filename) = path.file_name() {
                            let dst_path = orders_dst.join(filename);
                            if let Err(e) = fs::copy(&path, &dst_path) {
                                println!("cargo:warning=Failed to copy {}: {}", path.display(), e);
                            } else {
                                println!("cargo:warning=Copied {} to {}", path.display(), dst_path.display());
                            }
                        }
                    }
                }
            }
        }
    }
}
