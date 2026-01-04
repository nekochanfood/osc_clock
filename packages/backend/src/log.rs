use std::io::{ self, Write };

pub enum LogType {
    INFO,
    WARN,
    ERROR,
    EVENT,
    SEND,
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
