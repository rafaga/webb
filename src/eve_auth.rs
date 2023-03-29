use crate::esi::EsiData;
use rfesi::prelude::*;
//use rand::Rng;
use std::net::SocketAddr;
use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

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
            Ok::<_, Infallible>(service_fn(Self::hello_world))
        });

        let server = Server::bind(&addr).serve(make_svc);

        // Run this server for... forever!
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
        Ok(())
    }

    async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new("Hello, World".into()))
    }

}

impl Default for AuthService{
    fn default() -> Self {
        AuthService::new()
    }
}