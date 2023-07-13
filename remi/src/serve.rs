use std::convert::Infallible;

use remi_core::io::{Acceptor, AcceptorItem};
use remi_core::server::protocol::Protocol;
use remi_util::ext::AcceptorExt;
use tokio::macros::support::poll_fn;
use tower::Service;

pub async fn serve<A, M, P, S>(
    mut accpetor: A,
    mut make_service: M,
    mut protocol: P,
) -> Result<(), A::Error>
where
    A: Acceptor,
    A::Address: std::fmt::Debug,
    M: for<'a> Service<&'a AcceptorItem<A>, Response = S, Error = Infallible>,
    P: Protocol<A::Connection, S>,
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

        let (conn, addr) = accept.split();

        println!("Got connection from {:?}", addr);

        tokio::task::spawn(protocol.serve(conn, service));
    }

    Ok(())
}
