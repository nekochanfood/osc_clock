
use chrono::Local;
use rosc::{ OscMessage, OscType };
use serde::{Serialize, Deserialize};
use crate::{ config::CONFIG, log::{ print_flush, print_log, LogType }, unit::handle_unit };
use crate::order::{ Order};

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct SyncFlag: u32 {
        const MINUTE = 0b00000001;
        const HOUR   = 0b00000010;
        const DAY    = 0b00000100;
    }
}

pub struct BuilderParams {
    pub orders: Vec<Order>,
    pub sync_flag: SyncFlag,
}

pub fn build(params: BuilderParams) -> Vec<OscMessage> {
    let dt = Local::now();

    if CONFIG.lock().unwrap().show_debug_log {
        let str = t!(
            "debug_on_make_message",
            timestamp = dt.format("%Y-%m-%d %H:%M:%S.%f"),
            minute = format!("{:<5}", params.sync_flag.contains(SyncFlag::MINUTE)),
            hour = format!("{:<5}", params.sync_flag.contains(SyncFlag::HOUR)),
            date = format!("{:<5}", params.sync_flag.contains(SyncFlag::DAY))
        );
        print_flush(print_log(str, LogType::INFO));
    }

    let mut messages: Vec<OscMessage> = Vec::new();
    
    for order in params.orders.iter() {
        let msg = make_message(&order.address, vec![handle_unit(order.r#type, dt, params.sync_flag)]);
        if msg.args[0] != OscType::Nil {
            messages.push(msg);
        }
    }
    return messages;
}

pub fn make_message(addr: &str, args: Vec<OscType>) -> OscMessage {
    let message = OscMessage {
        addr: addr.to_string(),
        args: args,
    };
    return message;
}
