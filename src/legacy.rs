use crate::sender::sender;
use crate::receiver::receiver;

pub async fn start() {
    let sender_task = tokio::spawn(async move {
        sender().await;
    });

    let receiver_task = tokio::spawn(async move {
        receiver().await;
    });

    let _ = tokio::join!(sender_task, receiver_task);
}
