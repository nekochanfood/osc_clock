use std::io::{ self, Write };
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub enum LogType {
    INFO,
    WARN,
    ERROR,
    EVENT,
    SEND,
}

impl LogType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogType::INFO => "INFO",
            LogType::WARN => "WARN",
            LogType::ERROR => "ERROR",
            LogType::EVENT => "EVENT",
            LogType::SEND => "SEND",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub message: String,
    pub log_type: LogType,
    pub timestamp: String,
}

// Global log callback
type LogCallback = Box<dyn Fn(LogEntry) + Send + Sync>;
static LOG_CALLBACK: Lazy<Mutex<Option<LogCallback>>> = Lazy::new(|| Mutex::new(None));

/// Set a callback function to be called whenever a log message is generated
pub fn set_log_callback<F>(callback: F)
where
    F: Fn(LogEntry) + Send + Sync + 'static,
{
    *LOG_CALLBACK.lock().unwrap() = Some(Box::new(callback));
}

/// Clear the log callback
pub fn clear_log_callback() {
    *LOG_CALLBACK.lock().unwrap() = None;
}

pub fn print_log(str: String, log_type: LogType) -> String {
    let prefix;
    match log_type {
        LogType::INFO => {
            prefix = format!("[\x1b[{}mINFO\x1b[m]\t", 30 + 6);
        }
        LogType::WARN => {
            prefix = format!("[\x1b[{}mWARN\x1b[m]\t", 30 + 3);
        }
        LogType::ERROR => {
            prefix = format!("[\x1b[{}mERROR\x1b[m]\t", 30 + 1);
        }
        LogType::EVENT => {
            prefix = format!("[\x1b[{}mEVENT\x1b[m]\t", 30 + 5);
        }
        LogType::SEND => {
            prefix = format!("[\x1b[{}mSEND\x1b[m]\t", 30 + 2);
        }
    }
    return format!("{}{}\n", prefix, str);
}

pub fn print_flush(str: String) {
    println!("{}", str);
    match io::stdout().flush() {
        Ok(_) => {}
        Err(_) => std::process::exit(1),
    }
}

/// Log a message with the given type, and call the callback if set
pub fn log_message(message: String, log_type: LogType) {
    // Create log entry
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let entry = LogEntry {
        message: message.clone(),
        log_type: log_type.clone(),
        timestamp: timestamp.clone(),
    };
    
    // Call callback if set
    if let Some(callback) = LOG_CALLBACK.lock().unwrap().as_ref() {
        callback(entry);
    }
    
    // Print to console
    let formatted = print_log(message, log_type);
    print_flush(formatted);
}
