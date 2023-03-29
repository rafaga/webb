use crate::esi::EsiData;
use rfesi::prelude::*;
//use rand::Rng;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

static CONFIRM: &[u8] = b"<html><head><title>Telescope login</title><style>body{font-family: monospace;background-color: gray;color: whitesmoke;}</style></head><body><h1>Telescope</h1><p>Logged in!, now you can close this window safetly.</p></body></html>";
static NOT_VALID: &[u8] = b"Invalid Request";

pub struct AuthService{
    data: EsiData,
    esi: Option<Esi>,
}

impl AuthService{
    pub fn new() -> Self {
        AuthService{
            data: EsiData::new(),
            esi: None,
        }
    }

    pub fn connect(&mut self) -> Result<bool,EsiError>{
        let esi_obj = EsiBuilder::new()
            .user_agent(&self.data.user_agent)
            .client_id(&self.data.client_id)
            .client_secret(&self.data.secret_key)
            .callback_url(&self.data.callback_url)
            .build()?;
        self.esi = Some(esi_obj);
        Ok(true)
    }

    pub async fn create_server(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 56123));

        // A `Service` is needed for every connection, so this
        // creates one from our `hello_world` function.
        let make_svc = make_service_fn(|_conn| async {
            // service_fn converts our function into a `Service`
            Ok::<_, hyper::Error>(service_fn(Self::hello_world))
        });

        let server = Server::bind(&addr).serve(make_svc);

        // Run this server for... forever!
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
        Ok(())
    }

    async fn hello_world(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/login") => { 
                let pnq = req.uri().path_and_query();
                if let Some(params) = pnq.unwrap().query() {
                    let parameters = params.split("&").collect::<Vec<&str>>();
                    for param in parameters {
                        let p = param.split("=").collect::<Vec<&str>>();
                        match p[0] {
                            "code" => {

                            },
                            "random" => {

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
        }
    }

}

impl Default for AuthService{
    fn default() -> Self {
        AuthService::new()
    }
}