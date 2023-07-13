use remi::core::io::AcceptorItem;
use remi_tcp::TcpAcceptor;
use tower::service_fn;

#[tokio::main]
async fn main() {
    let acceptor = TcpAcceptor::bind("127.0.0.1:3434").await.unwrap();
    let make_svc = service_fn(|_: &AcceptorItem<TcpAcceptor>| async { Ok(()) });
    let server = remi_rsocket::RSocket::new();

    remi::serve(acceptor, make_svc, server).await.unwrap();
}
