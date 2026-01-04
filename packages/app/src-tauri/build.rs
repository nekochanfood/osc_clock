use std::path::Path;
use std::process::Command;
use std::fs;

fn main() {
    // Sync version from workspace Cargo.toml to tauri.conf.json
    sync_version();
    
    println!("cargo:rerun-if-changed=../../../assets/icons/icon.svg");
    
    // Generate icons from icon.svg (only in src-tauri/icons/)
    generate_icons();
    
    tauri_build::build()
}

fn sync_version() {
    // Read workspace version from root Cargo.toml
    let workspace_cargo_path = Path::new("../../../Cargo.toml");
    let workspace_cargo = fs::read_to_string(workspace_cargo_path)
        .expect("Failed to read workspace Cargo.toml");
    
    // Extract version from [workspace.package]
    let version = workspace_cargo
        .lines()
        .skip_while(|line| !line.contains("[workspace.package]"))
        .skip(1)
        .find(|line| line.trim().starts_with("version"))
        .and_then(|line| {
            line.split('=')
                .nth(1)
                .map(|v| v.trim().trim_matches('"').to_string())
        })
        .expect("Failed to find version in workspace Cargo.toml");
    
    // Read tauri.conf.json
    let tauri_config_path = Path::new("tauri.conf.json");
    let tauri_config = fs::read_to_string(tauri_config_path)
        .expect("Failed to read tauri.conf.json");
    
    // Parse and update version
    let mut config: serde_json::Value = serde_json::from_str(&tauri_config)
        .expect("Failed to parse tauri.conf.json");
    
    let current_version = config["version"].as_str().unwrap_or("0.0.0");
    
    if current_version != version {
        config["version"] = serde_json::Value::String(version.clone());
        
        // Write back to tauri.conf.json
        let updated_config = serde_json::to_string_pretty(&config)
            .expect("Failed to serialize tauri.conf.json");
        
        fs::write(tauri_config_path, updated_config)
            .expect("Failed to write tauri.conf.json");
        
        println!("cargo:warning=Synced version to {} in tauri.conf.json", version);
    }
    
    // Also update package.json version
    let package_json_path = Path::new("../package.json");
    if package_json_path.exists() {
        let package_json = fs::read_to_string(package_json_path)
            .expect("Failed to read package.json");
        
        let mut package: serde_json::Value = serde_json::from_str(&package_json)
            .expect("Failed to parse package.json");
        
        let pkg_version = package["version"].as_str().unwrap_or("0.0.0");
        
        if pkg_version != version {
            package["version"] = serde_json::Value::String(version.clone());
            
            let updated_package = serde_json::to_string_pretty(&package)
                .expect("Failed to serialize package.json");
            
            fs::write(package_json_path, updated_package)
                .expect("Failed to write package.json");
            
            println!("cargo:warning=Synced version to {} in package.json", version);
        }
    }
    
    println!("cargo:rerun-if-changed=../../../Cargo.toml");
}

fn generate_icons() {
    let icon_svg = Path::new("../../../assets/icons/icon.svg");
    
    // Check if icon.svg exists
    if !icon_svg.exists() {
        println!("cargo:warning=Icon source not found: {:?}", icon_svg);
        return;
    }
    
    // Always generate icons (overwrite existing ones)
    println!("cargo:warning=Generating icons from icon.svg...");
    
    // First try bun (if available)
    let bun_output = Command::new("bun")
        .args(&["run", "tauri", "icon", icon_svg.to_str().unwrap()])
        .current_dir("..")
        .output();
    
    let success = match bun_output {
        Ok(output) if output.status.success() => {
            println!("cargo:warning=Icons generated successfully with bun");
            true
        }
        _ => {
            // Fallback to cargo-tauri
            let cargo_output = Command::new("cargo")
                .args(&["tauri", "icon", icon_svg.to_str().unwrap()])
                .current_dir("..")
                .output();
            
            match cargo_output {
                Ok(output) if output.status.success() => {
                    println!("cargo:warning=Icons generated successfully with cargo-tauri");
                    true
                }
                Ok(_) | Err(_) => {
                    println!("cargo:warning=Icon generation skipped (bun/cargo-tauri not available)");
                    println!("cargo:warning=Run manually: cargo icons");
                    false
                }
            }
        }
    };
    
    if !success {
        println!("cargo:warning=Icons will need to be generated manually before building");
    }
    // Don't copy to assets/icons - keep them only in src-tauri/icons/
}
