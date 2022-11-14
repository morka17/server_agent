use tokio::sync::mpsc::Receiver;
use crate::Message;

pub async fn file_sink(mut channel: Receiver<Message>) {
    while let Some(msg) = channel.recv().await {
        println!("{:?}", msg);
    }
}