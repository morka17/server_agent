use tokio::sync::mpsc::Receiver;
use serde::{Serialize, Deserialize};


use std::fs::File;
use std::io::prelude::*;


pub async fn file_sink<T: std::fmt::Debug + Serialize>(mut channel: Receiver<T>) -> std::io::Result<()> {

    // Create a data bin file
    let mut file = File::create("data.bin")?;

    while let Some(msg) = channel.recv().await {
        println!("Writing to file {:?}", msg);

        file.write(&bincode::serialize(&msg).unwrap())?;
         
    }

    Ok(())
}