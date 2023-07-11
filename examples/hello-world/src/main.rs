#[tokio::main]
async fn main() {
    let acceptor = remi_tcp::TcpAcceptor::bind("127.0.0.1:3434").await.unwrap();
    remi::serve(acceptor).await.unwrap();
}
