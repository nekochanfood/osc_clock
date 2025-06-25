use crate::log::{print_flush, print_log, LogType};
use crate::unit::UnitType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    pub r#type: UnitType,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Orders {
    pub sender: Vec<Order>,
    pub handler: Vec<Order>,
}

pub static ORDERS: Lazy<Orders> = Lazy::new(|| load_orders());

pub fn init_orders() {
    Lazy::force(&ORDERS);
}

pub fn load_orders() -> Orders {
    let mut orders = Vec::new();
    let orders_dir = Path::new("orders");

    if !orders_dir.exists() {
        if let Err(e) = fs::create_dir_all(orders_dir) {
            eprintln!("Failed to create orders directory: {}", e);
            return split(get_fallback_orders());
        }
    }

    let mut found_file = false;

    if let Ok(entries) = fs::read_dir(orders_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                if fname.starts_with("orders_") && fname.ends_with(".json") {
                    found_file = true;
                    if let Ok(data) = fs::read_to_string(&path) {
                        if let Ok(mut file_orders) = serde_json::from_str::<Vec<Order>>(&data) {
                            orders.append(&mut file_orders);
                        }
                    }
                }
            }
        }
    }

    if !found_file {
        print!("Orders file not found. Do you want to create a orders file for OSC Clock? (Y/n): ");
        let mut input = String::new();
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).ok();
        let input = input.trim().to_lowercase();
        println!();
        if input == "y" || input == "yes" || input == "Y" {
            let default_path = orders_dir.join("orders_osc-clock.json");
            let default_orders: Vec<Order> = get_fallback_orders();
            if let Ok(json) = serde_json::to_string_pretty(&default_orders) {
                let _ = fs::write(&default_path, json);
            }
            print_flush(print_log("Orders file created".to_string(), LogType::INFO));
            return load_orders();
        } else {
            orders = get_fallback_orders();
            print_flush(print_log(
                "Using fallback orders".to_string(),
                LogType::WARN,
            ));
        }
    }
    split(orders)
}

fn split(orders: Vec<Order>) -> Orders {
    let mut sender = Vec::new();
    let mut handler = Vec::new();

    for order in orders {
        if order.r#type == UnitType::UpdateHandler {
            handler.push(order);
        } else {
            sender.push(order);
        }
    }

    Orders {
        sender,
        handler,
    }
}

pub fn get_fallback_orders() -> Vec<Order> {
    vec![
        Order {
            r#type: UnitType::SecondFloat,
            address: "/avatar/parameters/osc_clock@second_f".to_string(),
        },
        Order {
            r#type: UnitType::SecondInt,
            address: "/avatar/parameters/osc_clock@second_i".to_string(),
        },
        Order {
            r#type: UnitType::MinuteFloatMixed,
            address: "/avatar/parameters/osc_clock@minute_f".to_string(),
        },
        Order {
            r#type: UnitType::MinuteInt,
            address: "/avatar/parameters/osc_clock@minute_i".to_string(),
        },
        Order {
            r#type: UnitType::Hour24FloatMixed,
            address: "/avatar/parameters/osc_clock@hour24_f".to_string(),
        },
        Order {
            r#type: UnitType::Hour24Int,
            address: "/avatar/parameters/osc_clock@hour24_i".to_string(),
        },
        Order {
            r#type: UnitType::Hour12FloatMixed,
            address: "/avatar/parameters/osc_clock@hour12_f".to_string(),
        },
        Order {
            r#type: UnitType::Hour12Int,
            address: "/avatar/parameters/osc_clock@hour12_i".to_string(),
        },
        Order {
            r#type: UnitType::IsPm,
            address: "/avatar/parameters/osc_clock@hour_isPM".to_string(),
        },
        Order {
            r#type: UnitType::DayInt,
            address: "/avatar/parameters/osc_clock@day".to_string(),
        },
        Order {
            r#type: UnitType::DayOfWeekInt,
            address: "/avatar/parameters/osc_clock@dofw".to_string(),
        },
        Order {
            r#type: UnitType::MonthInt,
            address: "/avatar/parameters/osc_clock@month".to_string(),
        },
        Order {
            r#type: UnitType::Year,
            address: "/avatar/parameters/osc_clock@year".to_string(),
        },
        Order {
            r#type: UnitType::Year0,
            address: "/avatar/parameters/osc_clock@year_0".to_string(),
        },
        Order {
            r#type: UnitType::Year1,
            address: "/avatar/parameters/osc_clock@year_1".to_string(),
        },
        Order {
            r#type: UnitType::Year2,
            address: "/avatar/parameters/osc_clock@year_2".to_string(),
        },
        Order {
            r#type: UnitType::Year3,
            address: "/avatar/parameters/osc_clock@year_3".to_string(),
        },
        Order {
            r#type: UnitType::UpdateHandler,
            address: "/avatar/parameters/osc_clock@ForceSync".to_string(),
        },
    ]
}
