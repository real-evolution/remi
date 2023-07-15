use std::convert::Infallible;
use std::fmt::Display;

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
    A::Address: Display,
    M: for<'a> Service<&'a AcceptorItem<A>, Response = S, Error = Infallible>,
    P: Server<A::Connection, S>,
    P::MakeError: std::error::Error,
    S: Send + 'static,
    P::Service: Instance<S>,
    <P::Service as Service<S>>::Future: Send,
{
    loop {
        let Ok(accept) = accpetor.accept().await else {
            break;
        };

        tracing::debug!(
          address = %accept.address(),
          "accepted a new connection"
        );

        poll_fn(|cx| make_service.poll_ready(cx))
            .await
            .unwrap_or_else(|err| match err {});

        let service = make_service
            .call(&accept)
            .await
            .unwrap_or_else(|err| match err {});

        if let Err(err) = poll_fn(|cx| server.poll_ready(cx)).await {
            eprintln!("server poll_ready error: {}", err);
            continue;
        }

        let (conn, addr) = accept.split();

        let mut conn_service = match server.make_service(conn).await {
            | Ok(conn_service) => conn_service,
            | Err(err) => {
                tracing::warn!(
                    address = %addr,
                    error = %err,
                    "could not setup connection"
                );
                continue;
            }
        };

        tracing::info!(address = %addr, "serving connection");

        tokio::task::spawn(async move {
            match conn_service.call(service).await {
                | Ok(()) => {}
                | Err(_) => {}
            };
        });
    }

    Ok(())
}
