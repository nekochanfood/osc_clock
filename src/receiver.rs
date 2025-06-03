use chrono::Local;
use rosc::{ OscMessage, OscPacket, OscType };
use std::net::{ UdpSocket, SocketAddr };

use crate::config::{ Config, CONFIG };
use crate::log::{ print_log, print_flush, LogType };
use crate::message::{ build, BuilderParams };
use crate::sender::send;

pub async fn receiver() {
    let mut config = CONFIG.lock().unwrap().clone();

    let receiver_address: SocketAddr = (
        config.receiver_ip.to_string() +
        ":" +
        &config.receiver_port.to_string()
    )
        .parse()
        .expect(&print_log("Failed to parse address".to_string(), LogType::ERROR));
    let socket = UdpSocket::bind(receiver_address).expect(
        &print_log("Failed to bind socket".to_string(), LogType::ERROR)
    );

    print_flush(print_log(t!("listening_to_N", address = receiver_address), LogType::INFO));

    loop {
        let mut buf = [0; 2048];
        let (size, _) = socket
            .recv_from(&mut buf)
            .expect(&print_log(t!("failed_to_receive_data").to_string(), LogType::ERROR));
        match rosc::decoder::decode_udp(&buf[..size]) {
            Ok(packet) => {
                match packet {
                    (_, OscPacket::Message(msg)) => {
                        config = CONFIG.lock().unwrap().clone();
                        if check(msg.clone(), config.clone()) {
                            let sync_toggle = vec![true, true, true];
                            let messages = build(BuilderParams {
                                addresses: config.addresses.to_vec(),
                                sync_toggle,
                            });
                            for message in messages {
                                send(message, &config.sender_ip, config.sender_port);
                            }
                        }
                    }
                    (_, OscPacket::Bundle(bundle)) => {
                        print_flush(
                            print_log(
                                t!("received_osc_bundle", bundle = format!("{:?}", bundle)),
                                LogType::INFO
                            )
                        );
                    }
                }
            }
            Err(err) => {
                print_flush(
                    print_log(
                        t!("error_decoding_OSC_message", error = format!("{:?}", err)),
                        LogType::INFO
                    )
                );
            }
        }
    }
}

pub fn check(msg: OscMessage, config: Config) -> bool {
    let update: bool;

    match msg.args[0] {
        OscType::Bool(b) => {
            if b {
                update = true;
            } else {
                update = false;
            }
        }
        _ => {
            update = true;
        }
    }
    if update {
        for n in 0..config.update_handle_addresses.len() {
            if msg.addr.to_string() == config.update_handle_addresses[n].to_string() {
                print_flush(
                    print_log(
                        t!(
                            "on_receive_packet_from_specific_address",
                            address = msg.addr.to_string()
                        ),
                        LogType::EVENT
                    )
                );
                print_flush(
                    print_log(
                        t!(
                            "parameters_synced",
                            timestamp = Local::now().format("%Y-%m-%d %H:%M:%S")
                        ),
                        LogType::SEND
                    )
                );
                return true;
            }
        }
    }

    return false;
}
