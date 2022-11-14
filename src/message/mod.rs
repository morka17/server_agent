use serde::{Serialize, Deserialize};

pub mod message_generator;
pub mod message_recorder;


#[derive(Debug, Serialize, Deserialize)]
pub enum Message{
    Hello, 
    World,
}
