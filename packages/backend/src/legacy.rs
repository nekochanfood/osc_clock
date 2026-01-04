use rosc::OscMessage;

use crate::config::CONFIG;
use crate::sender::{ sender, send };
use crate::receiver::receiver;

pub async fn start() {
    let send = |m: OscMessage| {
        let config = CONFIG.lock().unwrap().clone();
        async move {
            send(m, &config.sender_ip, config.sender_port);
        }
    };

    let sender_task = tokio::spawn(async move {
        sender(send).await;
    });

    let receiver_task = tokio::spawn(async move {
        receiver().await;
    });

    let _ = tokio::join!(sender_task, receiver_task);
}
