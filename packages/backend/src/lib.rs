extern crate rosc;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

pub mod config;
pub mod log;
pub mod message;
pub mod order;
pub mod path;
pub mod unit;

pub mod legacy;
pub mod osc_query;
pub mod receiver;
pub mod sender;

pub mod recovery;

pub use config::{Config, CONFIG};
pub use order::{Order, Orders, ORDERS};

use tokio::sync::oneshot;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct OscClockService {
    running: Arc<AtomicBool>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl OscClockService {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            shutdown_tx: None,
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

        // Start the appropriate communication method
        let config = CONFIG.lock().unwrap().clone();
        
        if config.use_osc_query {
            // OSC Query mode
            osc_query::start().await?;
        } else {
            // Legacy OSC mode
            legacy::start().await;
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_running() {
            return Err("Service is not running".into());
        }

        self.running.store(false, Ordering::Relaxed);

        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

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
