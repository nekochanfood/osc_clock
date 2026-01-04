use rosc::OscMessage;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool};

use crate::config::CONFIG;
use crate::sender::{ sender, send };
use crate::receiver::receiver;

pub async fn start() {
    let running = Arc::new(AtomicBool::new(true));
    start_with_cancel(running).await;
}

pub async fn start_with_cancel(running: Arc<AtomicBool>) {
    let send = |m: OscMessage| {
        let config = CONFIG.lock().unwrap().clone();
        async move {
            send(m, &config.sender_ip, config.sender_port);
        }
    };

    let running_sender = running.clone();
    let sender_task = tokio::spawn(async move {
        sender(send, running_sender).await;
    });

    let running_receiver = running.clone();
    let receiver_task = tokio::spawn(async move {
        receiver(running_receiver).await;
    });

    let _ = tokio::join!(sender_task, receiver_task);
}
