use chrono::{ Local };
use vrchat_osc::{ models::OscRootNode, Error, ServiceType, VRChatOSC };
use rosc::{ OscMessage, OscPacket };

use crate::log::{ print_log, print_flush, LogType };
use crate::config::{ CONFIG };
use crate::sender::sender;

use crate::{ message, receiver::check, sender::{ send } };

pub async fn start() -> Result<(), Error> {
    let vrchat_osc = VRChatOSC::new().await?;

    vrchat_osc.on_connect(move |res| {
        match res {
            ServiceType::Osc(name, addr) => {
                print_flush(
                    print_log(
                        t!("on_connect_to_osc_server", name = name, address = addr),
                        LogType::INFO
                    )
                );
            }
            ServiceType::OscQuery(name, addr) => {
                print_flush(
                    print_log(
                        t!("on_connect_to_osc_query_server", name = name, address = addr),
                        LogType::INFO
                    )
                );
            }
        }
    }).await;

    let service_name = "osc-clock";
    let root_node = OscRootNode::new().with_avatar();
    vrchat_osc.register(&service_name, root_node, |packet| {
        if let OscPacket::Message(msg) = packet {
            let config = CONFIG.lock().unwrap().clone();
            if check(msg.clone(), config.clone()) {
                let sync_toggle = vec![true, true, true];
                let messages = message::build(message::BuilderParams {
                    addresses: config.addresses.to_vec(),
                    sync_toggle,
                });
                for message in messages {
                    send(message, &config.sender_ip, config.sender_port);
                }
            }
        }
    }).await?;

    print_flush(print_log(t!("osc_query_service_registered", name = &service_name), LogType::INFO));

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let cloned_vrchat_osc = vrchat_osc.clone();
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
                    print_flush(print_log(str, LogType::SEND));
                }
            }
        };
        sender(send).await;
    });

    tokio::signal::ctrl_c().await?;
    sender_task.abort();
    vrchat_osc.shutdown().await?;

    Ok(())
}
