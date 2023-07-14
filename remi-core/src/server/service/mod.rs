mod streaming;
mod unary;

use tower::{MakeService, Service};

pub use self::streaming::*;
pub use self::unary::*;

/// A trait to represent services that create servers that produce types that
/// implement [`Instance<Svc>`] trait.
pub trait Server<Conn, Svc>: MakeService<Conn, Svc>
where
    Self::Service: Instance<Svc>,
{
}

/// A trait to represent types that can run passed services.
pub trait Instance<Svc>: Service<Svc, Response = ()> + Send + 'static {}

impl<M, S, Conn, Svc> Server<Conn, Svc> for M
where
    M: MakeService<Conn, Svc, Service = S>,
    S: Instance<Svc>,
{
}

impl<S, Conn> Instance<Conn> for S where
    S: Service<Conn, Response = ()> + Send + 'static
{
}
