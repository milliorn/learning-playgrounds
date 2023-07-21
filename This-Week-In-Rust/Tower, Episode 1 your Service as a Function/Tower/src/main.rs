use std::{
    convert::Infallible,
    future::{ready, Ready},
    task::{Context, Poll},
};

use tower::Service;

pub struct EchoRequest(String);

pub struct EchoResponse(String);

pub struct EchoService;

/// Tower `Service` implementation for [EchoService]: always ready, never fails and responds
/// immediately with an echo, i.e. a response with the same content like the request.
impl Service<EchoRequest> for EchoService {
    type Response = EchoResponse;

    /// This service never fails.
    type Error = Infallible;

    /// This service responds immediately.
    type Future = Ready<Result<Self::Response, Self::Error>>;

    /// Always return `Poll::Ready`: this service is always ready.
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    /// Always return an echo, i.e. a response with the same content like the request.
    fn call(&mut self, req: EchoRequest) -> Self::Future {
        ready(Ok(EchoResponse(req.0)))
    }
}

fn main() {
    println!("Hello, world!");
}
