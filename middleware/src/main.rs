use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, get, web, App, Error, HttpServer};
use futures::future::{ok, Ready};
use futures::Future;

pub struct AppendHi;

impl<S, B> Transform<S> for AppendHi
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AppendHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AppendHiMiddleware { service })
    }
}

pub struct AppendHiMiddleware<S> {
    service: S,
}

impl<S, B> Service for AppendHiMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
    fn call(&mut self, req: Self::Request) -> Self::Future {
        println!("Start processing request");
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            println!("Hi from response");
            Ok(res)
        })
    }
}

#[get("/")]
async fn index() -> String {
    "Ok".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| App::new().service(index).wrap(AppendHi))
        .bind("127.0.0.1:4001")?
        .run()
        .await
}
