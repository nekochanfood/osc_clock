use chrono::{ Local, Timelike, Datelike };
use rosc::{ OscPacket, OscMessage };
use std::net::{ UdpSocket, SocketAddr };

use shared::{ log_message, LogType };
use crate::config::{ CONFIG };
use crate::order::ORDERS;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::message::{ build, BuilderParams, SyncFlag };

pub async fn sender<F, Fut>(s: F, running: Arc<AtomicBool>)
    where F: Fn(OscMessage) -> Fut, Fut: std::future::Future<Output = ()>
{
    let mut config = CONFIG.lock().unwrap().clone();

    if !config.use_osc_query {
        log_message(t!("sending_to_N", address = format!("{}:{}", config.sender_ip, config.sender_port)), LogType::INFO);
    }

    let mut dt = Local::now();

    let mut current_second = dt.second();
    let mut current_minute = u32::MAX;
    let mut current_hour = u32::MAX;
    let mut current_day = u32::MAX;

    while dt.second() == current_second && running.load(Ordering::Relaxed) {
        thread::sleep(std::time::Duration::from_millis(10));
        dt = Local::now();
    }

    while running.load(Ordering::Relaxed) {
        config = CONFIG.lock().unwrap().clone();

        while config.restrict_send_rate && dt.second() == current_second && running.load(Ordering::Relaxed) {
            thread::sleep(std::time::Duration::from_millis(config.check_rate_ms));
            dt = Local::now();
        }

        if !running.load(Ordering::Relaxed) { break; }


        let mut flag = SyncFlag::MINUTE | SyncFlag::HOUR | SyncFlag::DAY;

        if !config.send_all_value_every_time {
            if dt.minute() == current_minute {
                flag &= !SyncFlag::MINUTE;
            } else {
                current_minute = dt.minute();
            }
            if dt.hour() == current_hour {
                flag &= !SyncFlag::HOUR;
            } else {
                current_hour = dt.hour();
            }
            if dt.day() == current_day {
                flag &= !SyncFlag::DAY;
            } else {
                current_day = dt.day();
            }
        }
        let messages = build(BuilderParams {
            orders: ORDERS.clone().sender,
            sync_flag: flag,
        });
        for message in messages {
            s(message).await;
        }
        current_second = dt.second();
    }
    
    log_message("Sender task stopped".to_string(), LogType::INFO);
}

pub fn send(message: OscMessage, ip: &str, port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let addr = SocketAddr::new(ip.parse().unwrap(), port);

    let packet = OscPacket::Message(message);
    let encoded_packet = rosc::encoder::encode(&packet).unwrap();

    socket.send_to(&encoded_packet, addr).unwrap();

    if CONFIG.lock().unwrap().show_debug_log {
        let str = t!(
            if CONFIG.lock().unwrap().use_osc_query {
                "debug_on_send_message_osc_query"
            } else {
                "debug_on_send_message_legacy"
            },
            address = format!("{}:{}", ip, port),
            timestamp = Local::now().format("%Y-%m-%d %H:%M:%S.%f")
        );
        log_message(str, LogType::SEND);
    }
}
