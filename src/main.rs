use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast 
};



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, rx) = broadcast::channel::<String>(10);

  

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            let mut line = String::new();

            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap(); // read byte for the accepted TcpStream

                if bytes_read == 0 {
                    break;
                }

                tx.send(line.clone()).unwrap();


                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear()
            }
        });
    }
}
