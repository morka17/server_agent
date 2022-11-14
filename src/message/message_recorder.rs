use tokio::sync::{oneshot, mpsc::{Sender, channel}};
use failure::{Fallible};

use crate::Ctrl;
use crate::Message;
use crate::message::message_generator::CtrlResponse;
use crate::message_generator;
use crate::file_sink;


pub struct MessageRecorder{
    ctrl: Sender<(Ctrl, oneshot::Sender<CtrlResponse>)>
}

impl MessageRecorder {
    /// Spawns a MessageRecorder, will spawn
    /// two agents. A message generator and a file_sink 
    /// the agents in the form of a control channel 
    pub fn spawn() -> Fallible<MessageRecorder>{
        let (tx, rx) = channel::<Message>(10);

        let (ctx, crx) = channel::<(Ctrl, oneshot::Sender<CtrlResponse>)>(10);

        //message_generation -> file_sink
        tokio::spawn(message_generator(crx, tx));
        tokio::spawn(file_sink(rx));
 
        Ok(MessageRecorder{ctrl: ctx})

    }

    /// Sends a ctrl message to the spawned agents 
    pub async fn send_ctrl_message(&mut self, msg: Ctrl) -> Fallible<CtrlResponse> {

        let (rtx, rrx) = oneshot::channel();
        self.ctrl.send((msg, rtx)).await?;

        Ok(rrx.await?)
    }
}