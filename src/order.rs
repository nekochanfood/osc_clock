use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use serde_json;
use crate::unit::UnitType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    pub r#type: UnitType,
    pub address: String,
}

pub static ORDERS: Lazy<Vec<Order>> = Lazy::new(|| load_orders());

pub fn init_orders() {
    Lazy::force(&ORDERS);
}

pub fn load_orders() -> Vec<Order> {
    let mut orders = Vec::new();
    let orders_dir = Path::new("orders");

    if let Ok(entries) = fs::read_dir(orders_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                if fname.starts_with("orders_") && fname.ends_with(".json") {
                    if let Ok(data) = fs::read_to_string(&path) {
                        if let Ok(mut file_orders) = serde_json::from_str::<Vec<Order>>(&data) {
                            orders.append(&mut file_orders);
                        }
                    }
                }
            }
        }
    }
    orders
}