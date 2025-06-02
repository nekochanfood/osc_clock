use std::fmt::Debug;
use serde::{ Serialize, Deserialize };
use std::fs::File;
use std::{ fs };
use std::io::{ self, Read, Write };

use crate::log::{ print_log, LogType };
use crate::recovery::check_repair;
use crate::log::print_flush;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub language: String,
    pub sender_ip: String,
    pub sender_port: u16,
    pub receiver_ip: String,
    pub receiver_port: u16,
    pub show_debug_log: bool,
    pub send_all_value_every_time: bool,
    pub check_rate_ms: u64,
    pub restrict_send_rate: bool,
    pub use_osc_query: bool,
    pub addresses: Vec<String>,
    pub update_handle_addresses: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            sender_ip: "127.0.0.1".to_string(),
            sender_port: 9000,
            receiver_ip: "127.0.0.1".to_string(),
            receiver_port: 9001,
            show_debug_log: false,
            send_all_value_every_time: false,
            check_rate_ms: 1,
            restrict_send_rate: true,
            use_osc_query: false,
            addresses: vec![
                "/avatar/parameters/osc_clock@second_f".to_string(),
                "/avatar/parameters/osc_clock@second_i".to_string(),
                "/avatar/parameters/osc_clock@minute_f".to_string(),
                "/avatar/parameters/osc_clock@minute_i".to_string(),
                "/avatar/parameters/osc_clock@hour24_f".to_string(),
                "/avatar/parameters/osc_clock@hour24_i".to_string(),
                "/avatar/parameters/osc_clock@hour12_f".to_string(),
                "/avatar/parameters/osc_clock@hour12_i".to_string(),
                "/avatar/parameters/osc_clock@hour_isPM".to_string(),
                "/avatar/parameters/osc_clock@day".to_string(),
                "/avatar/parameters/osc_clock@dofw".to_string(),
                "/avatar/parameters/osc_clock@month".to_string(),
                "/avatar/parameters/osc_clock@year".to_string(),
                "/avatar/parameters/osc_clock@year_0".to_string(),
                "/avatar/parameters/osc_clock@year_1".to_string(),
                "/avatar/parameters/osc_clock@year_2".to_string(),
                "/avatar/parameters/osc_clock@year_3".to_string()
            ],
            update_handle_addresses: vec!["/avatar/parameters/osc_clock@ForceSync".to_string()],
        }
    }
}

pub fn get_fallback_config() -> Config {
    let config = Config::default();
    return config;
}

pub fn read_config_json(json_path: &str) -> Result<Config, io::Error> {
    let mut file = File::open(json_path)?;

    let mut json = String::new();
    let _ = file.read_to_string(&mut json)?;
    let config: Config = serde_json::from_str(&json)?;

    Ok(config)
}

pub fn repair_config_json() -> Result<bool, io::Error> {
    let mut file: File;
    let path = std::path::Path::new("./config.json");
    if path.is_file() {
        fs::remove_file("./config.json")?;
        file = File::create("./config.json")?;
    } else {
        file = File::create("./config.json")?;
    }
    let json = serde_json::to_string_pretty(&get_fallback_config())?;
    file.write_all(json.as_bytes()).expect(
        &print_log("Failed to write to file".to_string(), LogType::ERROR)
    );

    Ok(true)
}

pub fn load_config() -> Config {
    let config;
    check_repair();
    match read_config_json("./config.json") {
        Ok(result) => {
            config = result;
            rust_i18n::set_locale(&config.language);
        }
        Err(_error) => {
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
        print_flush(print_log(t!("warning_use_osc_query").to_string(), LogType::WARN));
    }
    
    return config;
}