use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        let (mut socket, _address) = listener.accept().await.unwrap();

        let (read, mut writer) = socket.split();

        let mut reader = BufReader::new(read);

        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await.unwrap();

            if bytes_read == 0 {
                break;
            }

            writer.write_all(&line.as_bytes()).await.unwrap();
            line.clear();
        }
    }
}
