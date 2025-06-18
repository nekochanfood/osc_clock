use chrono::{ Local, Timelike, Datelike };
use rosc::{ OscPacket, OscMessage };
use std::net::{ UdpSocket, SocketAddr };

use crate::log::{ print_log, print_flush, LogType };
use crate::config::{ CONFIG };
use crate::order::ORDERS;
use std::thread;
use crate::message::{ build, BuilderParams, SyncFlag };

pub async fn sender<F, Fut>(s: F)
    where F: Fn(OscMessage) -> Fut, Fut: std::future::Future<Output = ()>
{
    let mut config = CONFIG.lock().unwrap().clone();

    if !config.use_osc_query {
        print_flush(
            print_log(
                t!(
                    "sending_to_N",
                    address = format!("{}:{}", config.sender_ip, config.sender_port)
                ),
                LogType::INFO
            )
        );
    }

    let mut dt = Local::now();

    let mut current_second = dt.second();
    let mut current_minute = u32::MAX;
    let mut current_hour = u32::MAX;
    let mut current_day = u32::MAX;

    while dt.second() == current_second {
        thread::sleep(std::time::Duration::from_millis(10));
        dt = Local::now();
    }

    loop {
        config = CONFIG.lock().unwrap().clone();

        while config.restrict_send_rate && dt.second() == current_second {
            thread::sleep(std::time::Duration::from_millis(config.check_rate_ms));
            dt = Local::now();
        }

        let mut flag = SyncFlag::MINUTE | SyncFlag::HOUR | SyncFlag::DAY;

        if !config.send_all_value_every_time {
            if dt.minute() != current_minute {
            flag &= !SyncFlag::MINUTE;
            current_minute = dt.minute();
            }
            if dt.hour() != current_hour {
            flag &= !SyncFlag::HOUR;
            current_hour = dt.hour();
            }
            if dt.day() != current_day {
            flag &= !SyncFlag::DAY;
            current_day = dt.day();
            }
        }

        let messages = build(BuilderParams {
            orders: ORDERS.clone(),
            sync_flag: flag,
        });
        for message in messages {
            s(message).await;
        }
        current_second = dt.second();
    }
}

pub fn send(message: OscMessage, ip: &str, port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let addr = SocketAddr::new(ip.parse().unwrap(), port);

    let packet = OscPacket::Message(message);
    let encoded_packet = rosc::encoder::encode(&packet).unwrap();

    socket.send_to(&encoded_packet, addr).unwrap();

    if CONFIG.lock().unwrap().show_debug_log {
        let str = t!(
            "debug_on_send_message",
            address = format!("{}:{}", ip, port),
            timestamp = Local::now().format("%Y-%m-%d %H:%M:%S.%f")
        );
        print_flush(print_log(str, LogType::SEND));
    }
}
