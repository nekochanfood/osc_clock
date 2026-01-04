// OSC Clock GUI Application
// Tauri-based graphical interface

// In debug builds, show console window for Rust logging
// In release builds, hide console window
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::Emitter;
use std::sync::Arc;
use tokio::sync::Mutex;
use osc_clock::{OscClockService, Config, LogEntry, LogType, log_message, set_log_callback};

#[derive(Debug, Clone, Serialize)]
struct LogMessage {
    message: String,
    log_type: String,
    timestamp: String,
}

// Global service instance
static SERVICE: once_cell::sync::Lazy<Arc<Mutex<Option<OscClockService>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

#[tauri::command]
async fn start_service(app: tauri::AppHandle) -> Result<String, String> {
    log_message("start_service command called".to_string(), LogType::INFO);
    
    let mut service_lock = SERVICE.lock().await;
    
    if let Some(service) = service_lock.as_ref() {
        if service.is_running() {
            log_message("Service is already running".to_string(), LogType::WARN);
            return Err("Service is already running".to_string());
        }
    }
    
    // Set up log callback to emit events to frontend
    let app_clone = app.clone();
    set_log_callback(move |entry: LogEntry| {
        let log_msg = LogMessage {
            message: entry.message.clone(),
            log_type: entry.log_type.as_str().to_string(),
            timestamp: entry.timestamp.clone(),
        };
        let _ = app_clone.emit("log-message", log_msg);
    });
    
    log_message("Creating and starting OSC Clock service...".to_string(), LogType::INFO);
    
    // Create and start service
    let mut service = OscClockService::new();
    match service.start().await {
        Ok(_) => {
            log_message("Service started successfully".to_string(), LogType::INFO);
            *service_lock = Some(service);
            Ok("Service started successfully".to_string())
        }
        Err(e) => {
            log_message(format!("Failed to start service: {}", e), LogType::ERROR);
            Err(format!("Failed to start service: {}", e))
        }
    }
}

#[tauri::command]
async fn stop_service() -> Result<String, String> {
    log_message("stop_service command called".to_string(), LogType::INFO);
    
    let mut service_lock = SERVICE.lock().await;
    
    if let Some(service) = service_lock.as_mut() {
        // First call stop on the service
        match service.stop().await {
            Ok(_) => {
                log_message("Service stop signal sent".to_string(), LogType::INFO);
            }
            Err(e) => {
                log_message(format!("Failed to stop service: {}", e), LogType::ERROR);
                return Err(format!("Failed to stop service: {}", e));
            }
        }
    } else {
        log_message("Service is not running".to_string(), LogType::WARN);
        return Err("Service is not running".to_string());
    }
    
    // Drop the lock before sleeping
    drop(service_lock);
    
    // Wait for service to fully stop
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // Remove the service from the global state
    let mut service_lock = SERVICE.lock().await;
    *service_lock = None;
    
    log_message("Service stopped successfully".to_string(), LogType::INFO);
    Ok("Service stopped successfully".to_string())
}

#[tauri::command]
async fn restart_service(app: tauri::AppHandle) -> Result<String, String> {
    log_message("restart_service command called".to_string(), LogType::INFO);
    
    // Stop the service if running
    if let Err(e) = stop_service().await {
        if e != "Service is not running" {
            return Err(e);
        }
    }
    
    // Wait a bit before restarting
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // Start the service again
    start_service(app).await
}

#[tauri::command]
async fn is_service_running() -> bool {
    let service_lock = SERVICE.lock().await;
    
    if let Some(service) = service_lock.as_ref() {
        service.is_running()
    } else {
        false
    }
}

#[tauri::command]
async fn get_config() -> Result<Config, String> {
    let service_lock = SERVICE.lock().await;
    
    if let Some(service) = service_lock.as_ref() {
        Ok(service.get_config())
    } else {
        // Return default config if service is not running
        Ok(Config::default())
    }
}

#[tauri::command]
async fn update_config(config: Config) -> Result<String, String> {
    let mut service_lock = SERVICE.lock().await;
    
    if let Some(service) = service_lock.as_mut() {
        match service.update_config(config) {
            Ok(_) => Ok("Configuration updated successfully".to_string()),
            Err(e) => Err(format!("Failed to update configuration: {}", e))
        }
    } else {
        Err("Service is not running".to_string())
    }
}

fn main() {    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_service,
            stop_service,
            restart_service,
            is_service_running,
            get_config,
            update_config,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                use tauri::Manager;
                // Open DevTools automatically in debug builds
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                    log_message("DevTools opened for web debugging".to_string(), LogType::INFO);
                } else {
                    log_message("Warning: Could not find main window to open DevTools".to_string(), LogType::WARN);
                }
            }
            
            // Auto-start service on application startup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                log_message("Auto-starting service...".to_string(), LogType::INFO);
                match start_service(app_handle).await {
                    Ok(_) => log_message("Service auto-started successfully".to_string(), LogType::INFO),
                    Err(e) => log_message(format!("Failed to auto-start service: {}", e), LogType::ERROR),
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
