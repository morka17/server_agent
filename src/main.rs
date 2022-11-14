#!(allow_deadcode);

use tokio::sync::mpsc::channel;
mod message;
mod wait;
mod file_sink;

pub use message::{message_generator::message_generator, Message};
pub use file_sink::file_sink;
pub use wait::wait;



#[tokio::main]
async fn main() {
    let (tx, rx) = channel::<Message>(10);

    // message_generation -> file_sink
    tokio::spawn(message_generator(tx));
    tokio::spawn(file_sink(rx));

    wait(2000).await;
}
