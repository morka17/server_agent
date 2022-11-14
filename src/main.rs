#!(allow_deadcode);

mod message;
mod wait;
mod file_sink;

use tokio::sync::{oneshot, mpsc::{channel}};
use failure::{Fallible};

pub use message::{message_generator::{Ctrl, message_generator}, message_recorder::MessageRecorder,  Message};
pub use file_sink::file_sink;
pub use wait::wait;



#[tokio::main]
async fn main() ->  Fallible<()> {
   

    let mut msgr = MessageRecorder::spawn()?; 


    wait(2000).await;


    println!("Health message send...");
    let response = msgr.send_ctrl_message(Ctrl::Health).await?;
    println!("Received health response {:?}", response);


    wait(2000).await;

    println!("quiting message generator start....");
    let response = msgr.send_ctrl_message(Ctrl::Quit).await?;
    println!("quiting message generator end {:?}", response);

    wait(2000).await;

    println!("Existing program");

    Ok(())
}
