use chrono::{ Local };
use vrchat_osc::{ models::OscRootNode, Error, ServiceType, VRChatOSC };
use rosc::{ OscMessage, OscPacket };
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{ log_message, LogType };
use crate::config::{ CONFIG };
use crate::message::SyncFlag;
use crate::order::ORDERS;
use crate::receiver::check;
use crate::sender::{sender, send};
use crate::message::{ build, BuilderParams};

pub async fn start() -> Result<(), Error> {
    let running = Arc::new(AtomicBool::new(true));
    start_with_cancel(running).await
}

pub async fn start_with_cancel(running: Arc<AtomicBool>) -> Result<(), Error> {
    let vrchat_osc = VRChatOSC::new().await?;

    vrchat_osc.on_connect(move |res| {
        match res {
            ServiceType::Osc(name, addr) => {
                log_message(t!("on_connect_to_osc_server", name = name, address = addr), LogType::INFO);
            }
            ServiceType::OscQuery(name, addr) => {
                log_message(t!("on_connect_to_osc_query_server", name = name, address = addr), LogType::INFO);
            }
        }
    }).await;

    let service_name = "osc-clock";
    let root_node = OscRootNode::new().with_avatar();
    vrchat_osc.register(&service_name, root_node, |packet| {
        if let OscPacket::Message(msg) = packet {
            let config = CONFIG.lock().unwrap().clone();
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
    }).await?;
    log_message(t!("osc_query_service_registered", name = &service_name), LogType::INFO);

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let cloned_vrchat_osc = vrchat_osc.clone();
    let running_sender = running.clone();
    let sender_task = tokio::spawn(async move {
        let send = |m: OscMessage| {
            let cloned_vrchat_osc = cloned_vrchat_osc.clone();
            async move {
                cloned_vrchat_osc
                    .send(OscPacket::Message(m.clone()), "VRChat-Client-*").await
                    .unwrap();
                if CONFIG.lock().unwrap().show_debug_log {
                    let str = t!(
                        "debug_on_send_message_osc_query",
                        timestamp = Local::now().format("%Y-%m-%d %H:%M:%S.%f")
                    );
                    log_message(str, LogType::SEND);
                }
            }
        };
        sender(send, running_sender).await;
    });

    // Wait for running flag to become false
    while running.load(Ordering::Relaxed) {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    log_message("OSC Query service stopping...".to_string(), LogType::INFO);
    sender_task.abort();
    vrchat_osc.shutdown().await?;

    Ok(())
}
