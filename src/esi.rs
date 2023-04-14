use rfesi::prelude::*;
use futures::executor::block_on;
use tokio::time::{Instant, timeout_at};
use tokio::time::Duration;
use std::net::SocketAddr;
use hyper::Server;
use crate::auth_service::MakeSvc;
use crate::database::{Character, Alliance, Corporation};
use chrono::{DateTime,NaiveDateTime};
use chrono::Utc;



pub struct EsiManager{
    pub esi: Esi,
    pub characters: Vec<Character>,
}

impl EsiManager {

    pub fn new(useragent: &str, client_id: &str, client_secret: &str, callback_url: &str, scope: Vec<&str>) -> Self {

        let esi = EsiBuilder::new()
            .user_agent(useragent)
            .client_id(client_id)
            .client_secret(client_secret)
            .callback_url(callback_url)
            .scope(scope.join(" ").as_str())
            .build().unwrap();

        EsiManager {
            esi,
            characters: Vec::new(),
        }
    }

    #[tokio::main]
    pub async fn auth_user(&mut self,port: u16) -> Result<Option<Character>, Box<dyn std::error::Error + Send + Sync>> {
        let addr: SocketAddr = ([127, 0, 0, 1], port).into();
        let (tx, rx) = tokio::sync::oneshot::channel::<(String,String)>();
        crate::SHARED_TX.lock().await.replace(tx);
        let mut result = (String::new(),String::new());
        let server = Server::bind(&addr)
            .serve(MakeSvc::new())
            .with_graceful_shutdown(async {
                let msg = rx.await.ok();
                if let Some(values) = msg {
                    result = values;
                }
            });
        
        if let Err(err) = timeout_at(Instant::now() + Duration::from_secs(300), server).await {
            eprintln!("{}",err);
            return Ok(None);
        };

        let mut player = Character::new();
        block_on(async {    
            let claims = self.esi.authenticate(result.0.as_str()).await;
            let data = claims.unwrap().unwrap();
            //character name
            player.name = data.name;
            //character id
            let split:Vec<&str> = data.sub.split(":").collect();
            player.id = split[2].as_ptr() as u64;
            if player.auth != None {
                // owner
                player.auth.as_mut().unwrap().owner = data.owner;
                //jti
                player.auth.as_mut().unwrap().jti= data.jti;
                //expiration Date
                let naive_datetime = NaiveDateTime::from_timestamp_opt(data.exp, 0);
                player.auth.as_mut().unwrap().expiration = Some(DateTime::from_utc(naive_datetime.unwrap(), Utc));
            }
            let asyncdata = self.esi.group_character().get_public_info(player.id as u64).await;
            let mut ids:(u64,u64) = (0,0);
            if let Ok(public_data) = asyncdata {
                ids.0 = public_data.alliance_id;
                ids.1 = public_data.corporation_id;
            }
            if let Ok(tcorp) = self.esi.group_corporation().get_public_info(ids.0).await{
                let corp = Corporation{
                    id:ids.0,
                    name: tcorp.name,
                };
                player.corp = Some(corp);
            }
            if let Ok(talliance) = self.esi.group_alliance().get_info(ids.1).await{
                let alliance = Alliance{
                    id: ids.1,
                    name: talliance.name,
                };
                player.alliance = Some(alliance);
            }
            if let Ok(photo) = self.esi.group_character().get_portrait(player.id).await {
                player.photo= Some(photo.px64x64);
            };
        });
        Ok(Some(player))  
    }
}