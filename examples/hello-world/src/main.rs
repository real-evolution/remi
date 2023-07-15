use std::time::Duration;

use remi::core::io::AcceptorItem;
use remi_tcp::TcpAcceptor;
use tower::service_fn;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt().pretty().finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let acceptor = TcpAcceptor::bind("127.0.0.1:3434").await.unwrap();
    let make_svc = service_fn(|_: &AcceptorItem<TcpAcceptor>| async { Ok(()) });
    let server = remi_rsocket::RSocketServer::builder()
        .max_lifetime(Duration::from_secs(30))
        .stream()
        .build();

    remi::serve(acceptor, make_svc, server).await.unwrap();
}
