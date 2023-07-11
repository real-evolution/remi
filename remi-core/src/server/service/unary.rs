use futures::Future;
use tower::Service;

use crate::message::Message;

pub trait UnaryService<R: Message> {
    type Error;
    type Response: Message;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&mut self, request: R) -> Self::Future;
}

impl<T, Req, Res> UnaryService<Req> for T
where
    T: Service<Req, Response = Res>,
    Req: Message,
    Res: Message,
{
    type Error = T::Error;
    type Future = T::Future;
    type Response = Res;

    fn call(&mut self, request: Req) -> Self::Future {
        Service::call(self, request)
    }
}
