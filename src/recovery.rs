use crate::log::{ print_log, print_flush, LogType };
use crate::config::{ repair_config_json };
use std::env;
use std::io::{ self, Write };

pub fn check_repair() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if args[1] == "repair" {
            match repair_config_json() {
                Ok(_) => {
                    print_flush(print_log(t!("repair_success").to_string(), LogType::INFO));
                }
                Err(_error) => {
                    print_flush(print_log(t!("repair_failed"), LogType::ERROR));
                }
            }
            print!("{}", t!("press_any_key_to_continue"));
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect(&t!("failed_to_read_line"));
            return;
        }
    }
}
