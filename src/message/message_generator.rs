use tokio::{
    select,
    sync::{
        mpsc::{Receiver, Sender},
        oneshot,
    },
};

use crate::wait;
use crate::Message;

///# Ctrl channel
/// Help in checking the state of the message generator
///
/// it check if the message generator is
/// - Health -> Healthy or UnHealthy
///
/// Ctrl Channel also help in [`Quit`] of the message generator
#[derive(Debug)]
pub enum Ctrl {
    Quit,
    Health,
}

#[derive(Debug)]
pub enum CtrlResponse {
    Quit(QuitResponse),
    Health(HealthResponse)

}

#[derive(Debug)]
pub enum QuitResponse {
    Ok, 
}

#[derive(Debug)]
pub enum HealthResponse{
    Healthy,
    UnHealthy,
}

pub async fn message_generator(mut ctrl: Receiver<(Ctrl, oneshot::Sender<CtrlResponse>)>, channel: Sender<Message>) {
    loop {
        select! {
            msg = channel.send(Message::Hello) => match  msg  {
                Ok(()) => wait(100).await,
                Err(_) => {
                    eprintln!("Error sending message");
                    break;
                }
            },

            ctl = ctrl.recv() => match ctl {
                Some((Ctrl::Quit, rtx)) => {
                    rtx.send(CtrlResponse::Quit(QuitResponse::Ok)).expect("unable to respond to ctrl message");
                    break;
                },
                Some((Ctrl::Health, rtx)) => {
                    rtx.send(CtrlResponse::Health(HealthResponse::Healthy)).expect("unable to respond the ctrl health message");
                },
                None => break,
            }

        }
    }
}
