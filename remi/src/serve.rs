use std::{convert::Infallible, error::Error};

use remi_core::edge::Acceptor;
use remi_util::ext::AcceptorExt;
use tower::Service;

/// Serve connections using the given acceptor and service facotry.
pub async fn serve<A, M, S>(
    mut acceptor: A,
    mut make_service: M,
) -> Result<(), A::Error>
where
    A: Acceptor,
    M: for<'a> Service<&'a mut A::Conn, Error = Infallible, Response = S>,
    S: Service<(), Response = (), Error = Box<dyn Error>> + Send + 'static,
    S::Future: Send,
{
    loop {
        let mut conn = acceptor.accept().await?;
        let mut service = make_service
            .call(&mut conn)
            .await
            .unwrap_or_else(|err| match err {});

        tokio::spawn(async move {
            if let Err(err) = service.call(()).await {
                dbg!("Error: {}", err);
            }
        });
    }
}
