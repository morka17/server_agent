use tokio::sync::mpsc::{Sender};
use crate::Message;
use crate::wait;

pub async fn message_generator(channel: Sender<Message>) {
    loop {
        match channel.send(Message::Hello).await {
            Ok(()) => wait(100).await,
            Err(_) => {
                eprintln!("Error sending message");
                break;
            }
        }
    }
}
