#!/usr/bin/env rust
//! Icon Generator
//! Generates all required icon formats from icon.svg
//! Cross-platform replacement for generate-icons.ps1

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::fs;

fn main() {
    let workspace_root = get_workspace_root();
    let icon_svg = workspace_root.join("assets/icons/icon.svg");
    
    // Check if icon.svg exists
    if !icon_svg.exists() {
        eprintln!("Error: {} not found!", icon_svg.display());
        exit(1);
    }
    
    println!("Icon source: {}", icon_svg.display());
    
    // Check if icons already exist and are up to date
    let app_dir = workspace_root.join("packages/app");
    let src_icons = app_dir.join("src-tauri/icons");
    
    if src_icons.exists() && !needs_regeneration(&icon_svg, &src_icons) {
        println!("✅ Icons are up to date, skipping generation");
        return;
    }
    
    if !app_dir.exists() {
        eprintln!("Error: app directory not found!");
        exit(1);
    }
    
    println!("Generating icons...\n");
    
    // Try bun first, then cargo
    let success = generate_with_bun(&app_dir, &icon_svg) 
        || generate_with_cargo(&app_dir, &icon_svg);
    
    if !success {
        eprintln!("\nError: Failed to generate icons!");
        eprintln!("Make sure 'bun' or 'cargo-tauri' is installed.");
        exit(1);
    }
    
    // Icons are now in packages/app/src-tauri/icons/
    // They stay there and are used by the build system
    println!("\n✨ Icon generation complete!");
    println!("Icons generated in: packages/app/src-tauri/icons/");
}

fn needs_regeneration(icon_svg: &Path, icons_dir: &Path) -> bool {
    // Check if icon.svg is newer than generated icons
    let svg_modified = fs::metadata(icon_svg)
        .and_then(|m| m.modified())
        .ok();
    
    if let Some(svg_time) = svg_modified {
        // Check modification time of any icon file
        if let Ok(entries) = fs::read_dir(icons_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if svg_time <= modified {
                            return false; // Icons are newer
                        }
                    }
                }
            }
        }
    }
    
    true // Need regeneration
}

fn get_workspace_root() -> PathBuf {
    // Get current exe directory and go up to workspace root
    let mut current = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| env::current_dir().unwrap());
    
    // Try to find workspace root by looking for Cargo.toml with [workspace]
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
            // Fallback: assume we're in scripts/icon-generator
            return env::current_dir()
                .unwrap()
                .parent()
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| env::current_dir().unwrap());
        }
    }
}

fn generate_with_bun(app_dir: &Path, icon_svg: &Path) -> bool {
    println!("Trying bun tauri icon...");
    
    let output = Command::new("bun")
        .args(&["tauri", "icon", icon_svg.to_str().unwrap()])
        .current_dir(app_dir)
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            println!("✅ Icons generated with bun");
            true
        }
        Ok(output) => {
            eprintln!("bun failed: {}", String::from_utf8_lossy(&output.stderr));
            false
        }
        Err(_) => {
            println!("bun not available");
            false
        }
    }
}

fn generate_with_cargo(app_dir: &Path, icon_svg: &Path) -> bool {
    println!("Trying cargo tauri icon...");
    
    let output = Command::new("cargo")
        .args(&["tauri", "icon", icon_svg.to_str().unwrap()])
        .current_dir(app_dir)
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            println!("✅ Icons generated with cargo-tauri");
            true
        }
        Ok(output) => {
            eprintln!("cargo-tauri failed: {}", String::from_utf8_lossy(&output.stderr));
            false
        }
        Err(_) => {
            println!("cargo-tauri not available");
            false
        }
    }
}
