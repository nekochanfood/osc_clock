use chrono::{ Local, Timelike, Datelike };
use rosc::{ OscPacket, OscMessage, OscType };
use std::net::{ UdpSocket, SocketAddr };
use crate::log::{ print_log, print_flush, LogType };

pub fn composition(
    addresses: Vec<String>,
    ip: String,
    port: u16,
    sync_toggle: Vec<bool>,
    show_debug_log: bool
) {
    let dt = Local::now();

    let mut messages: Vec<OscMessage> = addresses
        .iter()
        .map(|addr| OscMessage {
            addr: addr.clone(),
            args: vec![],
        })
        .collect();

    // second
    let second = dt.second();

    messages.push(
        make_message(&addresses[0], vec![OscType::Float(((second as f32) / 60.0) as f32)])
    );
    messages.push(make_message(&addresses[1], vec![OscType::Int(second as i32)]));

    // minute
    let minute = dt.minute();
    messages.push(
        make_message(&addresses[2], vec![OscType::Float(((minute as f32) / 60.0) as f32)])
    );
    if sync_toggle[0] == true {
        messages.push(make_message(&addresses[3], vec![OscType::Int(minute as i32)]));
    }

    if sync_toggle[1] == true {
        // hour24
        let hour24 = dt.hour();
        messages.push(
            make_message(&addresses[4], vec![OscType::Float(((hour24 as f32) / 24.0) as f32)])
        );
        messages.push(make_message(&addresses[5], vec![OscType::Int(hour24 as i32)]));

        //  hour12 & isPM
        let is_pm = dt.hour12();
        let hour12 = if is_pm.1 == 12 { 0 } else { is_pm.1 };

        messages.push(
            make_message(
                &addresses[6],
                vec![
                    OscType::Float(
                        (((hour12 as f32) / 12.0) as f32) + (minute as f32) / 60.0 / 12.0
                    )
                ]
            )
        );
        messages.push(make_message(&addresses[7], vec![OscType::Int(hour12 as i32)]));
        messages.push(make_message(&addresses[8], vec![OscType::Bool(is_pm.0 as bool)]));
    }

    if sync_toggle[2] == true {
        // day
        let day = dt.day();

        messages.push(make_message(&addresses[9], vec![OscType::Int(day as i32)]));

        // dofw
        let dofw = dt.weekday();

        messages.push(make_message(&addresses[10], vec![OscType::Int(dofw as i32)]));

        // month
        let month = dt.month();

        messages.push(make_message(&addresses[11], vec![OscType::Int(month as i32)]));

        // year
        let year = dt.year();

        messages.push(make_message(&addresses[12], vec![OscType::Int(year as i32)]));

        // splitted year
        messages.push(make_message(&addresses[13], vec![OscType::Int((year / 1000) as i32)]));
        messages.push(
            make_message(&addresses[14], vec![OscType::Int(((year % 1000) / 100) as i32)])
        );
        messages.push(make_message(&addresses[15], vec![OscType::Int(((year % 100) / 10) as i32)]));
        messages.push(make_message(&addresses[16], vec![OscType::Int((year % 10) as i32)]));
    }

    for message in messages {
        send(message, &ip, port);
    }

    if show_debug_log {
        let str = t!(
            "debug_on_send_message",
            address = format!("{}:{}", ip, port),
            timestamp = dt.format("%Y-%m-%d %H:%M:%S.%f"),
            minute = format!("{:<5}", sync_toggle[0]),
            hour = format!("{:<5}", sync_toggle[1]),
            date = format!("{:<5}", sync_toggle[2])
        );
        print_flush(print_log(str, LogType::SEND));
    }
}

// 送信用
pub fn send(message: OscMessage, ip: &str, port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let addr = SocketAddr::new(ip.parse().unwrap(), port);

    let packet = OscPacket::Message(message);
    let encoded_packet = rosc::encoder::encode(&packet).unwrap();

    socket.send_to(&encoded_packet, addr).unwrap();
}


pub fn make_message(addr: &str, args: Vec<OscType>) -> OscMessage {
    let message = OscMessage {
        addr: addr.to_string(),
        args: args,
    };
    return message;
}