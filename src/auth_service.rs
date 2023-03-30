

use hyper::service::Service;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

static CONFIRM: &[u8] = b"<html><head><title>Telescope login</title><style>body{font-family: monospace;background-color: gray;color: whitesmoke;}</style></head><body><h1>Telescope</h1><p>Logged in!, now you can close this window safetly.</p></body></html>";
static NOT_VALID: &[u8] = b"Invalid Request";

pub struct AuthService{
    characterid: usize,
    code: usize,
    random: usize,
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
                    let parameters = params.split("&").collect::<Vec<&str>>();
                    for param in parameters {
                        let p = param.split("=").collect::<Vec<&str>>();
                        match p[0] {
                            "code" => {
                                self.code = p[1].parse().unwrap();
                            },
                            "random" => {
                                self.random = p[1].parse().unwrap();
                            },
                            _ => ()
                        }
                    }
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
    pub characterid: usize,
    pub code: usize,
    pub random: usize,

}

impl MakeSvc {
    pub fn new(characterid: usize) -> Self {
        MakeSvc {
            characterid,
            code: 0,
            random: 0,
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
        let characterid = self.characterid;
        let code = self.code;
        let random = self.random;
        let fut = async move { Ok(AuthService{ characterid, code, random}) };
        self.characterid = characterid;
        self.code = code;
        self.random = random;
        Box::pin(fut)
    }
}

pub fn open_auth_service() -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 56123).into();
    
    let server = Server::bind(&addr).serve(MakeSvc::new(0));
    println!("Listening on http://{}", addr);

    async {
        let k = server.await;
    };
    Ok(true)
}
