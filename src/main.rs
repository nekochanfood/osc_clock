extern crate rosc;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod config;
use config::{ init_config, CONFIG};

mod log;

mod recovery;

mod sender;
mod receiver;
use rosc::OscPacket;
use sender::{sender, send};
use receiver::{receiver, check};
use vrchat_osc::{models::OscRootNode, VRChatOSC};

mod message;

static VERSION: f32 = 1.0;

#[tokio::main]
async fn main() {
    // Title
    print!("OSC Clock v{0:.1}\n", VERSION);
    print!("{}\n\n", t!("press_ctrl+c_to_exit"));

    // Load configuration
    init_config();

    if CONFIG.lock().unwrap().use_osc_clock {
        let vrchat_osc = VRChatOSC::new().await;

        let root_node = OscRootNode::new().with_avatar();
        match vrchat_osc {
            Ok(vrchat_osc_instance) => {
                vrchat_osc_instance.register("test_service", root_node, |packet| {
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
                    } else {
                        eprintln!("Received non-message packet: {:?}", packet);
                    }
                }).await.unwrap();
            }
            Err(e) => {
                eprintln!("Failed to initialize VRChatOSC: {:?}", e);
                return;
            }
        }
    } else {
        let sender_task = tokio::spawn(async move {
            sender().await;
        });

        let receiver_task = tokio::spawn(async move {
            receiver().await;
        });

        let _ = tokio::join!(sender_task, receiver_task);
    }
}
