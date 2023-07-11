use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8001").await?;
    println!("Start listening");
    loop {
        let (mut socket, addr) = match listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                println!("Error accepting connection: {}", e);
                continue;
            }
        };
        println!("Accepted connection from {}", addr);
        let ftr = async move {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            socket
                .write_all("Hello incoming connection!".as_bytes())
                .await
        };
        tokio::task::spawn(ftr);
    }
}
