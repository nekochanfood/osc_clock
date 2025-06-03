use chrono::{ Local, Timelike, Datelike };
use rosc::{ OscPacket, OscMessage };
use std::net::{ UdpSocket, SocketAddr };

use crate::log::{ print_log, print_flush, LogType };
use crate::config::{ CONFIG };
use std::thread;
use crate::message::{ build, BuilderParams };

pub async fn sender() {
    let mut config = CONFIG.lock().unwrap().clone();

    print_flush(
        print_log(
            t!("sending_to_N", address = format!("{}:{}", config.sender_ip, config.sender_port)),
            LogType::INFO
        )
    );

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

        let send_minute: bool;
        let send_hour: bool;
        let send_day: bool;

        while config.restrict_send_rate && dt.second() == current_second {
            thread::sleep(std::time::Duration::from_millis(config.check_rate_ms));
            dt = Local::now();
        }

        if config.send_all_value_every_time {
            send_minute = true;
            send_hour = true;
            send_day = true;
        } else {
            if dt.minute() != current_minute {
                send_minute = true;
                current_minute = dt.minute();
            } else {
                send_minute = false;
            }
            if dt.hour() != current_hour {
                send_hour = true;
                current_hour = dt.hour();
            } else {
                send_hour = false;
            }
            if dt.day() != current_day {
                send_day = true;
                current_day = dt.day();
            } else {
                send_day = false;
            }
        }

        let sync_toggle = vec![send_minute, send_hour, send_day];
        let messages = build(BuilderParams {
            addresses: config.addresses.to_vec(),
            sync_toggle,
        });
        for message in messages {
            send(message, &config.sender_ip, config.sender_port);
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
            timestamp = Local::now().format("%Y-%m-%d %H:%M:%S.%f")
        );
        print_flush(print_log(str, LogType::SEND));
    }
}
