// Removed: use shared::print_flush;
use shared::{log_message, LogType};
use crate::path::get_exe_relative_path;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Write};
use std::sync::Mutex;
use std::{fs, vec};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub language: String,
    pub use_osc_query: bool,
    pub sender_ip: String,
    pub sender_port: u16,
    pub receiver_ip: String,
    pub receiver_port: u16,
    pub show_debug_log: bool,
    pub send_all_value_every_time: bool,
    pub check_rate_ms: u64,
    pub restrict_send_rate: bool,
    pub config_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigStatus {
    #[serde(rename = "Passed")]
    Passed,
    #[serde(rename = "Lacked")]
    Lacked,
    #[serde(rename = "Fallback")]
    Fallback,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Missing")]
    Missing,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            use_osc_query: false,
            sender_ip: "127.0.0.1".to_string(),
            sender_port: 9000,
            receiver_ip: "127.0.0.1".to_string(),
            receiver_port: 9001,
            show_debug_log: false,
            send_all_value_every_time: false,
            check_rate_ms: 1,
            restrict_send_rate: true,
            config_status: format!("{:?}", ConfigStatus::Fallback),
        }
    }
}

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(load_config()));

pub fn init_config() {
    Lazy::force(&CONFIG);
}

pub fn get_fallback_config() -> Config {
    let config = Config::default();
    return config;
}

fn merge_json(a: &mut serde_json::Value, b: &serde_json::Value) {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            for (k, v) in b_map {
                merge_json(a_map.entry(k).or_insert(Value::Null), v);
            }
        }
        (a_slot, b_val) => {
            *a_slot = b_val.clone();
        }
    }
}

pub fn read_config_json(json_path: &str, complement: bool) -> Result<Config, io::Error> {
    let mut file = File::open(json_path)?;

    let mut judge: ConfigStatus = ConfigStatus::Passed;
    _ = judge;

    let mut json = String::new();

    let mut config = Config::default();
    let mut default_value = serde_json::to_value(&config)?;

    if let Err(io_error) = file.read_to_string(&mut json) {
        judge = match io_error.kind() {
            ErrorKind::NotFound => ConfigStatus::Missing,
            _ => ConfigStatus::Failed,
        };
    } else {
        let partial: serde_json::Value = serde_json::from_str(&json)?;
        if check_itgr(&partial, &default_value, &["config_status"]) {
            if complement {
                merge_json(&mut default_value, &partial);
                judge = ConfigStatus::Passed;
            } else {
                default_value = partial;
                judge = ConfigStatus::Passed;
            }
        } else {
            if complement {
                merge_json(&mut default_value, &partial);
                judge = ConfigStatus::Lacked;
            } else {
                default_value = partial;
                judge = ConfigStatus::Lacked;
            }
        }
    }

    let status = &serde_json::from_str(&format!("{{\"config_status\":\"{:?}\"}}", judge))?;
    merge_json(&mut default_value, status);
    config = serde_json::from_value(default_value)?;

    Ok(config)
}

fn check_itgr(
    partial: &serde_json::Value,
    default: &serde_json::Value,
    exceptions: &[&str],
) -> bool {
    let mut count = 0;
    if let (Value::Object(partial_map), Value::Object(default_map)) = (partial, default) {
        for k in default_map.keys() {
            if exceptions.contains(&k.as_str()) {
                continue;
            }
            if !partial_map.contains_key(k) {
                log_message(
                    format!("Lacked property found: {}", k),
                    LogType::WARN,
                );
                count += 1;
            }
        }
    }
    return count == 0;
}

pub fn repair_config_json(force: bool) -> Result<bool, io::Error> {
    let mut file: File;
    let config_path = get_exe_relative_path("config.json");
    let path = config_path.as_path();
    if path.is_file() && !force {
        match read_config_json(path.to_str().unwrap(), true) {
            Ok(result) => {
                rust_i18n::set_locale(&result.language);
            }
            Err(_error) => {
                return repair_config_json(true);
            }
        }
    } else {
        let mut config = Config::default();
        if path.exists() {
            match read_config_json(path.to_str().unwrap(), true) {
                Ok(result) => {
                    config = result;
                    log_message(format!("Config file found"), LogType::INFO);
                }
                Err(_error) => {
                    log_message(format!("Failed to load config file"), LogType::WARN);
                }
            }
            fs::remove_file(path)?;
        }
        file = File::create(&config_path)?;
        let json =
            serde_json::to_string_pretty(&validate(config, vec!["config_status".to_string()]))?;

        file.write_all(json.as_bytes()).unwrap_or_else(|_| {
            log_message("Failed to write to file".to_string(), LogType::ERROR);
            panic!("Failed to write to file");
        });
    }

    Ok(true)
}

fn validate(config: Config, exclusions: Vec<String>) -> serde_json::Value {
    let mut value = serde_json::to_value(config).unwrap();
    if let serde_json::Value::Object(ref mut map) = value {
        for key in exclusions {
            map.remove(&key);
        }
    }
    value
}

fn load_config() -> Config {
    let config;
    let config_path = get_exe_relative_path("config.json");
    match read_config_json(config_path.to_str().unwrap(), true) {
        Ok(result) => {
            config = result;
            rust_i18n::set_locale(&config.language);
            if config.config_status == format!("{:?}", ConfigStatus::Lacked) {
                log_message(format!(
                            "Deprecated or lacked properties config has been detected. Use \"osc_clock.exe --repair\" to fix the config file."
                        ), LogType::WARN);
            }
        }
        Err(_error) => {
            // Handle error when loading config file
            match _error.kind() {
                ErrorKind::NotFound => {
                    print!("Config file not found. Do you want to create a config file? (Y/n): ");
                    let mut input = String::new();
                    io::stdout().flush().ok();
                    io::stdin().read_line(&mut input).ok();
                    let input = input.trim().to_lowercase();
                    println!();
                    if input == "y" || input == "yes" || input == "Y" {
                        let _ = repair_config_json(true);
                        config = load_config();
                        log_message("Config file created".to_string(), LogType::INFO);
                    } else {
                        config = get_fallback_config();
                        log_message("Using fallback config".to_string(), LogType::WARN);
                    }
                }
                _ => {
                    log_message(t!("failed_to_load_config").to_string(), LogType::WARN);
                    log_message(t!("how_to_repair_config").to_string(), LogType::INFO);
                    config = get_fallback_config();
                }
            }
        }
    }

    if config.send_all_value_every_time {
        log_message(t!("warning_send_all_value").to_string(), LogType::WARN);
    }

    if !config.restrict_send_rate {
        log_message(t!("warning_restrict_send_rate").to_string(), LogType::WARN);
    }

    if config.check_rate_ms == 0 {
        log_message(t!("warning_check_rate_ms_zero").to_string(), LogType::WARN);
    } else if config.check_rate_ms > 100 {
        log_message(t!("warning_check_rate_ms_too_much").to_string(), LogType::WARN);
    }

    if config.use_osc_query {
        log_message(t!("warning_osc_query_enabled").to_string(), LogType::INFO);
    }

    return config;
}
