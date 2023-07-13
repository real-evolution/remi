use std::future::Future;
use std::task::Poll;

pub trait Protocol<Conn, Svc> {
    type Error: Send + 'static;
    type Future: Future<Output = Result<(), Self::Error>> + Send + 'static;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;

    fn serve(&mut self, conn: Conn, service: Svc) -> Self::Future;
}
