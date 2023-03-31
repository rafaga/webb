use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::net::SocketAddr;

use std::sync::Arc;
use hyper::service::Service;
use hyper::{Request, Response};
use hyper::{Body, Method,  Server, StatusCode};

static CONFIRM: &[u8] = b"<html><head><title>Telescope login</title><style>body{font-family: monospace;background-color: gray;color: whitesmoke;}</style></head><body><h1>Telescope</h1><p>Logged in!, now you can close this window safetly.</p></body></html>";
static NOT_VALID: &[u8] = b"Invalid Request";

pub struct AuthService{
    tx: Arc<tokio::sync::oneshot::Sender<(String,String)>>,
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
                    let parameters = params.split("&").collect::<Vec<&str>>();
                    for param in parameters {
                        let p = param.split("=").collect::<Vec<&str>>();
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
                        let tx = Arc::clone(&self.tx);
                        tx.send(message);
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

struct MakeSvc {
    tx: Arc<tokio::sync::oneshot::Sender<(String,String)>>,
}

impl MakeSvc {
    pub fn new(sender: Arc<tokio::sync::oneshot::Sender<(String,String)>>) -> Self {
        MakeSvc {
            tx: sender
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
        let tx = Arc::clone(&self.tx);
        let fut = async move { Ok(AuthService{ tx }) };
        Box::pin(fut)
    }
}

#[tokio::main]
pub async fn open_auth_service() -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 56123).into();
    let (tx, rx) = tokio::sync::oneshot::channel::<(String,String)>();
    let shared_tx = Arc::new(tx);
    let server = Server::bind(&addr)
        .serve(MakeSvc::new(Arc::clone(&shared_tx)))
        .with_graceful_shutdown(async {
            rx.await.ok();
        });
    
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(true)
}
