use std::fmt::Debug;
use serde::{ Serialize, Deserialize };
use serde_json::Value;
use std::fs::File;
use std::{ fs };
use std::io::{ self, Read, Write };
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::log::{ print_log, LogType };
use crate::recovery::check_repair;
use crate::log::print_flush;

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
    pub update_handle_addresses: Vec<String>,
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
            update_handle_addresses: vec!["/avatar/parameters/osc_clock@ForceSync".to_string()],
            config_status: format!("{:?}", ConfigStatus::Fallback),
        }
    }
}

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| { Mutex::new(load_config()) });

pub fn init_config() {
    Lazy::force(&CONFIG);
}

pub fn get_fallback_config() -> Config {
    let config = Config::default();
    return config;
}

fn merge_json(a: &mut serde_json::Value, b: &serde_json::Value) {
    use serde_json::Value;
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

    if let Err(_) = file.read_to_string(&mut json) {
        judge = ConfigStatus::Lacked;
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
    exceptions: &[&str]
) -> bool {
    let mut count = 0;
    if let (Value::Object(partial_map), Value::Object(default_map)) = (partial, default) {
        for k in default_map.keys() {
            if exceptions.contains(&k.as_str()) {
                continue;
            }
            if !partial_map.contains_key(k) {
                print_flush(print_log(format!("Lacked property found: {}", k), LogType::WARN));
                count += 1;
            }
        }
        
    }
    return count == 0;
}

pub fn repair_config_json(force: bool) -> Result<bool, io::Error> {
    let mut file: File;
    let path = std::path::Path::new("./config.json");
    if path.is_file()  && !force {
        match read_config_json(path.to_str().unwrap(), true) {
            Ok(result) => {
                rust_i18n::set_locale(&result.language);
            }
            Err(_error) => {
                return repair_config_json(true)
            }
        }
    } else {
        if path.exists() {
            fs::remove_file(path)?;
        }
        file = File::create("./config.json")?;
        let json = serde_json::to_string_pretty(&get_fallback_config())?;

        file.write_all(json.as_bytes()).expect(
            &print_log("Failed to write to file".to_string(), LogType::ERROR)
        );
    }

    Ok(true)
}

fn load_config() -> Config {
    let config;
    check_repair();
    match read_config_json("./config.json", true) {
        Ok(result) => {
            config = result;
            rust_i18n::set_locale(&config.language);
            if config.config_status == format!("{:?}", ConfigStatus::Lacked) {
                print_flush(print_log(format!("Deprecated or lacked properties config has been detected. Use \"osc_clock.exe --repair\" to fix the config file."), LogType::WARN));
            }
        }
        Err(_error) => {
            print_flush(format!("{:?}\n\n\n", _error));
            print_flush(print_log(t!("failed_to_load_config").to_string(), LogType::WARN));
            print_flush(print_log(t!("how_to_repair_config").to_string(), LogType::INFO));
            config = get_fallback_config();
        }
    }

    if config.send_all_value_every_time {
        print_flush(print_log(t!("warning_send_all_value").to_string(), LogType::WARN));
    }

    if !config.restrict_send_rate {
        print_flush(print_log(t!("warning_restrict_send_rate").to_string(), LogType::WARN));
    }

    if config.check_rate_ms == 0 {
        print_flush(print_log(t!("warning_check_rate_ms_zero").to_string(), LogType::WARN));
    } else if config.check_rate_ms > 100 {
        print_flush(print_log(t!("warning_check_rate_ms_too_much").to_string(), LogType::WARN));
    }

    if config.use_osc_query {
        print_flush(print_log(t!("warning_osc_query_enabled").to_string(), LogType::INFO));
    }

    return config;
}
