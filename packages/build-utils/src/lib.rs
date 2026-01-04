use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

/// Get the workspace root by looking for Cargo.toml with [workspace]
pub fn get_workspace_root() -> PathBuf {
    let mut current = std::env::current_dir().unwrap();
    
    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(content) = fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    return current;
                }
            }
        }
        
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            panic!("Could not find workspace root");
        }
    }
}

/// Generate icons from icon.svg using tauri icon command
/// Returns true if successful, false otherwise
pub fn generate_icons_tauri(working_dir: &Path, icon_svg: &Path) -> bool {
    println!("cargo:warning=Generating icons from {}...", icon_svg.display());
    
    // Try bun first
    let bun_result = Command::new("bun")
        .args(&["run", "tauri", "icon", icon_svg.to_str().unwrap()])
        .current_dir(working_dir)
        .output();
    
    match bun_result {
        Ok(output) if output.status.success() => {
            println!("cargo:warning=✅ Icons generated with bun");
            return true;
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                println!("cargo:warning=Bun icon generation failed: {}", stderr.trim());
            }
        }
        Err(_) => {
            println!("cargo:warning=Bun not available, trying cargo-tauri...");
        }
    }
    
    // Fallback to cargo-tauri
    let cargo_result = Command::new("cargo")
        .args(&["tauri", "icon", icon_svg.to_str().unwrap()])
        .current_dir(working_dir)
        .output();
    
    match cargo_result {
        Ok(output) if output.status.success() => {
            println!("cargo:warning=✅ Icons generated with cargo-tauri");
            return true;
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                println!("cargo:warning=Cargo-tauri icon generation failed: {}", stderr.trim());
            }
        }
        Err(_) => {
            println!("cargo:warning=Cargo-tauri not available");
        }
    }
    
    println!("cargo:warning=❌ Icon generation failed - using existing icons if available");
    false
}

/// Copy a file, overwriting if it exists
pub fn copy_file_overwrite(src: &Path, dst: &Path) -> std::io::Result<()> {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(src, dst)?;
    Ok(())
}

/// Embed Windows icon for exe
#[cfg(windows)]
pub fn embed_windows_icon(icon_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut res = winres::WindowsResource::new();
    res.set_icon(icon_path.to_str().unwrap());
    res.compile()?;
    println!("cargo:warning=✅ Windows icon embedded from {}", icon_path.display());
    Ok(())
}

#[cfg(not(windows))]
pub fn embed_windows_icon(_icon_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=Icon embedding is only supported on Windows");
    Ok(())
}
