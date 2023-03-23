use hyper::{Body, Error, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use crate::esi::EsiData;
use rfesi::prelude::*;

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

    pub async fn create_server(self) {
        let (url,_random_data) = self.esi.unwrap().get_authorize_url().unwrap();
        let _resp = open::that(url);
        // Construct our SocketAddr to listen on...
        let addr = ([127, 0, 0, 1], 56123).into();

        // And a MakeService to handle each connection...
        let make_svc = make_service_fn(|_| async {
            Ok::<_, Error>(service_fn(|_req| async {
                Ok::<_, Error>(Response::new(Body::from("Telescope")))
            }))
        });

        // Then bind and serve...
        let server = Server::bind(&addr)
            .serve(make_svc);

        // Run forever-ish...
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }

        //server.
    }
}

impl Default for AuthService{
    fn default() -> Self {
        AuthService::new()
    }
}