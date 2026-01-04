pub mod log;

pub use log::{LogEntry, LogType, log_message, set_log_callback, clear_log_callback, print_log, print_flush};

/// Get the path to shared assets
pub fn get_assets_path() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets")
}

/// Get the path to shared icons
pub fn get_icons_path() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/icons")
}
