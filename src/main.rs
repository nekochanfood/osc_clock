extern crate rosc;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod config;
use config::{ load_config };

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

    // Load configuration
    let config = load_config();

    let sender_config = config.clone();
    let receiver_config = config.clone();

    let sender_task = tokio::spawn(async move {
        sender(sender_config).await;
    });

    let receiver_task = tokio::spawn(async move {
        receiver(receiver_config).await;
    });

    let _ = tokio::join!(sender_task, receiver_task);

    print!("{}\n\n",t!("press_ctrl+c_to_exit"));
}
