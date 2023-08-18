use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use hyper::service::Service;
use hyper::{Request, Response};
use hyper::{Body, Method, StatusCode};

use futures::executor::block_on;

static CONFIRM: &[u8] = b"<html><head><title>Telescope login</title><style>body{font-family: monospace;background-color: gray;color: whitesmoke;}</style></head><body><h1>Telescope</h1><p>Logged in!, now you can close this window safetly.</p></body></html>";
static NOT_VALID: &[u8] = b"Invalid Request";

pub (crate) struct AuthService{
}

impl AuthService{
}

impl Service<Request<Body>> for AuthService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let res = match (req.method(), req.uri().path()) {
            (&Method::GET, "/login") => { 
                let pnq = req.uri().path_and_query();
                if let Some(params) = pnq.unwrap().query() {
                    let mut message:(String,String)=(String::new(),String::new()); 
                    let parameters = params.split('&').collect::<Vec<&str>>();
                    for param in parameters {
                        let p = param.split('=').collect::<Vec<&str>>();
                        match p[0] {
                            "code" => {
                                message.0 = p[1].to_string();
                            },
                            "state" => {
                                message.1 = p[1].to_string();
                            },
                            _ => ()
                        }
                    }
                    if !message.0.is_empty() && !message.1.is_empty() {
                        block_on(async {
                            if let Some(tx) = crate::SHARED_TX.lock().await.take() {
                                let _res = tx.send(message.clone());
                            }
                        });
                        Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(CONFIRM.into())
                        .unwrap())
                    } else {
                        Ok(Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(NOT_VALID.into())
                        .unwrap())
                    }
                } else {
                    Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(NOT_VALID.into())
                    .unwrap())
                }

            },
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()),
        };
        Box::pin(async { res })
    }

}

pub (crate) struct MakeSvc {
}

impl MakeSvc {
    pub fn new() -> Self {
        MakeSvc {
        }
    }
}

impl<T> Service<T> for MakeSvc {
    type Response = AuthService;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let fut = async move { Ok(AuthService{}) };
        Box::pin(fut)
    }
}