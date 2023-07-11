use std::future::Future;

use tokio_stream::Stream;
use tower::Service;

use crate::message::Message;

pub trait ServerStreamingService<R: Message> {
    type Error;
    type Response: Message;
    type ResponseStream: Stream<Item = Result<Self::Response, Self::Error>>;
    type Future: Future<Output = Result<Self::ResponseStream, Self::Error>>;

    fn call(&mut self, request: R) -> Self::Future;
}

impl<T, Req, Res, S> ServerStreamingService<Req> for T
where
    T: Service<Req, Response = S>,
    S: Stream<Item = Result<Res, T::Error>>,
    Req: Message,
    Res: Message,
{
    type Error = T::Error;
    type Future = T::Future;
    type Response = Res;
    type ResponseStream = S;

    #[inline]
    fn call(&mut self, request: Req) -> Self::Future {
        Service::call(self, request)
    }
}

pub trait ClientStreamingService<R>
where
    R: Stream<Item = Result<Self::Request, Self::Error>>,
{
    type Error;
    type Request: Message;
    type Response: Message;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&mut self, request: R) -> Self::Future;
}

impl<T, Req, S, Res> ClientStreamingService<S> for T
where
    T: Service<S, Response = Res>,
    S: Stream<Item = Result<Req, T::Error>>,
    Req: Message,
    Res: Message,
{
    type Error = T::Error;
    type Future = T::Future;
    type Request = Req;
    type Response = Res;

    #[inline]
    fn call(&mut self, request: S) -> Self::Future {
        Service::call(self, request)
    }
}

pub trait StreamingService<R>
where
    R: Stream<Item = Result<Self::Request, Self::Error>>,
{
    type Error;
    type Request: Message;
    type Response: Message;
    type ResponseStream: Stream<Item = Result<Self::Response, Self::Error>>;
    type Future: Future<Output = Result<Self::ResponseStream, Self::Error>>;

    fn call(&mut self, request: R) -> Self::Future;
}

impl<T, Req, S1, Res, S2> StreamingService<S1> for T
where
    T: Service<S1, Response = S2>,
    S1: Stream<Item = Result<Req, T::Error>>,
    S2: Stream<Item = Result<Res, T::Error>>,
    Req: Message,
    Res: Message,
{
    type Error = T::Error;
    type Future = T::Future;
    type Request = Req;
    type Response = Res;
    type ResponseStream = S2;

    #[inline]
    fn call(&mut self, request: S1) -> Self::Future {
        Service::call(self, request)
    }
}
