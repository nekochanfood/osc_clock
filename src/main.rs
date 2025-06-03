extern crate rosc;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod config;
use config::{ init_config, CONFIG, set_sender_port };

mod log;

mod recovery;

mod sender;
mod receiver;
use sender::sender;
use receiver::receiver;

mod message;

static VERSION: f32 = 1.0;

#[tokio::main]
async fn main() {
    // Title
    print!("OSC Clock v{0:.1}\n", VERSION);
    print!("{}\n\n",t!("press_ctrl+c_to_exit"));

    // Load configuration
    init_config();

    let sender_task = tokio::spawn(async move {
        sender().await;
    });

    let receiver_task = tokio::spawn(async move {
        receiver().await;
    });

    let _ = tokio::join!(sender_task, receiver_task);
}
