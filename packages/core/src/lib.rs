
#[macro_use]
extern crate rust_i18n;
i18n!("locales");

pub mod config;
pub mod message;
pub mod order;
pub mod path;
pub mod unit;
pub mod log;

pub mod legacy;
pub mod osc_query;
pub mod receiver;
pub mod sender;

pub mod recovery;

pub use config::{Config, CONFIG};
pub use order::{Order, Orders, ORDERS};

pub use log::{LogEntry, LogType, log_message, set_log_callback, clear_log_callback};

use tokio::task::JoinHandle;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct OscClockService {
    running: Arc<AtomicBool>,
    task_handle: Option<JoinHandle<()>>,
}

impl OscClockService {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            task_handle: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_running() {
            return Err("Service is already running".into());
        }

        // Initialize configuration and orders
        config::init_config();
        order::init_orders();

        self.running.store(true, Ordering::Relaxed);

        let running = self.running.clone();
        
        // Start the appropriate communication method in a separate task
        let handle = tokio::spawn(async move {
            let config = CONFIG.lock().unwrap().clone();
            
            if config.use_osc_query {
                // OSC Query mode
                if let Err(e) = osc_query::start_with_cancel(running.clone()).await {
                    log_message(format!("OSC Query service error: {}", e), LogType::ERROR);
                }
            } else {
                // Legacy OSC mode
                legacy::start_with_cancel(running.clone()).await;
            }
            
            log_message("Service task completed".to_string(), LogType::INFO);
        });
        
        self.task_handle = Some(handle);

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_running() {
            return Err("Service is not running".into());
        }

        log_message("Stopping service...".to_string(), LogType::INFO);
        
        // Set running flag to false to signal tasks to stop
        self.running.store(false, Ordering::Relaxed);

        // Wait for the task to complete with a timeout
        if let Some(handle) = self.task_handle.take() {
            match tokio::time::timeout(tokio::time::Duration::from_secs(5), handle).await {
                Ok(result) => {
                    if let Err(e) = result {
                        log_message(format!("Task join error: {}", e), LogType::WARN);
                    }
                }
                Err(_) => {
                    log_message("Service stop timeout - task may still be running".to_string(), LogType::WARN);
                }
            }
        }

        log_message("Service stopped".to_string(), LogType::INFO);
        Ok(())
    }

    pub fn get_config(&self) -> Config {
        CONFIG.lock().unwrap().clone()
    }

    pub fn update_config(&mut self, new_config: Config) -> Result<(), Box<dyn std::error::Error>> {
        *CONFIG.lock().unwrap() = new_config.clone();
        
        // Save to file
        use std::fs::File;
        use std::io::Write;
        
        let config_path = path::get_exe_relative_path("config.json");
        let mut file = File::create(&config_path)?;
        let json = serde_json::to_string_pretty(&new_config)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }

    /// Get the current orders
    pub fn get_orders(&self) -> Orders {
        ORDERS.clone()
    }
}

impl Default for OscClockService {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = OscClockService::new();
    service.start().await?;
    Ok(())
}
