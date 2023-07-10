use std::convert::Infallible;
use std::error::Error;

use remi_core::io::{Acceptor, AcceptorState};
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
    while let AcceptorState::Running = acceptor.state() {
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

    Ok(())
}
