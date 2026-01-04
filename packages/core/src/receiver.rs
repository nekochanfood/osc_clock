use chrono::Local;
use rosc::{ OscMessage, OscPacket, OscType };
use std::net::{ UdpSocket, SocketAddr };
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::config::{ CONFIG };
use shared::{ log_message, LogType };
use crate::message::{ build, BuilderParams, SyncFlag };
use crate::order::{Order, ORDERS};
use crate::sender::send;

pub async fn receiver(running: Arc<AtomicBool>) {
    let mut config = CONFIG.lock().unwrap().clone();

    let receiver_address: SocketAddr = (
        config.receiver_ip.to_string() +
        ":" +
        &config.receiver_port.to_string()
    )
        .parse()
        .unwrap_or_else(|_| {
            log_message("Failed to parse address".to_string(), LogType::ERROR);
            panic!("Failed to parse address");
        });
    let socket = UdpSocket::bind(receiver_address).unwrap_or_else(|_| {
        log_message("Failed to bind socket".to_string(), LogType::ERROR);
        panic!("Failed to bind socket");
    });

    // Set socket to non-blocking mode so we can check the running flag
    socket.set_nonblocking(true).expect("Failed to set non-blocking");

    log_message(t!("listening_to_N", address = receiver_address), LogType::INFO);

    while running.load(Ordering::Relaxed) {
        let mut buf = [0; 2048];
        match socket.recv_from(&mut buf) {
            Ok((size, _)) => {
                match rosc::decoder::decode_udp(&buf[..size]) {
                    Ok(packet) => {
                        match packet {
                            (_, OscPacket::Message(msg)) => {
                                config = CONFIG.lock().unwrap().clone();
                                if check(msg.clone(), ORDERS.clone().handler) {
                                    let flag = SyncFlag::MINUTE | SyncFlag::HOUR | SyncFlag::DAY;
                                    let messages = build(BuilderParams {
                                        orders: ORDERS.clone().sender,
                                        sync_flag: flag,
                                    });
                                    for message in messages {
                                        send(message, &config.sender_ip, config.sender_port);
                                    }
                                }
                            }
                            (_, OscPacket::Bundle(bundle)) => {
                                log_message(
                                    format!("Received OSC bundle: {:?}", bundle),
                                    LogType::WARN,
                                );
                            }
                        }
                    }
                    Err(e) => {
                        log_message(
                            format!("Failed to decode OSC packet: {}", e),
                            LogType::ERROR,
                        );
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available, sleep briefly and check running flag
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
            Err(e) => {
                log_message(format!("Socket error: {}", e), LogType::ERROR);
                break;
            }
        }
    }
    
    log_message("Receiver task stopped".to_string(), LogType::INFO);
}

pub fn check(msg: OscMessage, order: Vec<Order>) -> bool {
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
        for n in 0..order.len() {
            if msg.addr.to_string() == order[n].address.to_string() {
                log_message(
                    t!(
                        "on_receive_packet_from_specific_address",
                        address = msg.addr.to_string()
                    ),
                    LogType::EVENT
                );
                log_message(
                    t!(
                        "parameters_synced",
                        timestamp = Local::now().format("%Y-%m-%d %H:%M:%S")
                    ),
                    LogType::SEND
                );
                return true;
            }
        }
    }

    return false;
}
