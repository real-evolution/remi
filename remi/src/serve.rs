use std::convert::Infallible;

use remi_core::io::{Acceptor, AcceptorItem};
use remi_core::server::service::{Instance, Server};
use remi_util::ext::AcceptorExt;
use tokio::macros::support::poll_fn;
use tower::Service;

pub async fn serve<A, M, P, S>(
    mut accpetor: A,
    mut make_service: M,
    mut server: P,
) -> Result<(), A::Error>
where
    A: Acceptor,
    M: for<'a> Service<&'a AcceptorItem<A>, Response = S, Error = Infallible>,
    P: Server<A::Connection, S, MakeError = Infallible>,
    S: Send + 'static,
    P::Service: Instance<S>,
    <P::Service as Service<S>>::Future: Send,
{
    loop {
        let Ok(accept) = accpetor.accept().await else {
            break;
        };

        poll_fn(|cx| make_service.poll_ready(cx))
            .await
            .unwrap_or_else(|err| match err {});

        let service = make_service
            .call(&accept)
            .await
            .unwrap_or_else(|err| match err {});

        poll_fn(|cx| server.poll_ready(cx))
            .await
            .unwrap_or_else(|err| match err {});

        let (conn, _) = accept.split();

        let mut conn_service = server
            .make_service(conn)
            .await
            .unwrap_or_else(|err| match err {});

        tokio::task::spawn(async move {
            match conn_service.call(service).await {
                | Ok(()) => {}
                | Err(_) => {}
            };
        });
    }

    Ok(())
}
