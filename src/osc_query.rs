use vrchat_osc::{ models::OscRootNode, Error, ServiceType, VRChatOSC };
use rosc::{ OscPacket };
use chrono::{ Local, Timelike, Datelike };

use crate::log::{ print_log, print_flush, LogType };
use crate::config::{ CONFIG };
use std::{ thread };
use crate::message::{ build, BuilderParams };

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
        let mut config = CONFIG.lock().unwrap().clone();
        let _ = config;

        let mut dt = Local::now();

        // 比較用
        let mut current_second = dt.second();
        let mut current_minute = u32::MAX;
        let mut current_hour = u32::MAX;
        let mut current_day = u32::MAX;

        // .000 秒まで待つ
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
                cloned_vrchat_osc
                    .send(OscPacket::Message(message.clone()), "VRChat-Client-*").await
                    .unwrap();
            }
            current_second = dt.second();
        }
    });

    tokio::signal::ctrl_c().await?;
    sender_task.abort();
    vrchat_osc.shutdown().await?;

    Ok(())
}
