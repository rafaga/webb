use crate::esi::EsiData;
use rfesi::prelude::*;
//use rand::Rng;
use std::net::SocketAddr;
use std::error::Error;
use hyper::body::Buf;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
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
    
        let listener = TcpListener::bind(addr).await?;
        println!("Listening on http://{}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
    
            tokio::task::spawn(async move {
                if let Err(err) = Http::new().serve_connection(stream, service_fn(Self::auth_handler)).await {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }
        Ok(())
    }

    pub async fn auth_handler( req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
        

        match (req.method(), req.uri().path()) {
            (&Method::GET, "/login") => {
                let path = req.uri().path_and_query();
                if let Some(pnq) = path{
                    let query = pnq.query().to_owned();
                    let query_segments = query.unwrap().split("&").collect::<Vec<&str>>();
                    for param in query_segments {
                        let item = param.split("=").collect::<Vec<&str>>();
                        if item[0] == "code" {
                            
                        }
                    }
                    if query_segments.len() <= 2 {
                        let res = get_car_list();
                        return Ok(res);
                    }
                    let car_id = path_segments[2];
                    if car_id.trim().is_empty() {
                        let res = get_car_list();
                        return Ok(res);
                    } else {
                        // code to fill whenever path is /cars/:id
                    }
                }  
            },
    
            (&Method::POST, "/login") => Ok(Response::new(Body::from("POST login"))),
    
            // Return the 404 Not Found for other routes.
            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }

    fn get_car_list() -> Response<Body> {
        let resp = String::from("<html><head></head><body><h1>OK</h1></body></html>");
    
        match serde_json::to_string(&resp) {
            Ok(json) => Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(INTERNAL_SERVER_ERROR.into())
                .unwrap(),
        }
    }
}

impl Default for AuthService{
    fn default() -> Self {
        AuthService::new()
    }
}