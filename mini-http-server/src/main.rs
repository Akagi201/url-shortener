use tokio::net::TcpListener;

mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            handler::handle_request(stream).await;
        });
    }
}